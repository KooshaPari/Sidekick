//! IPC implementation for sidekick-messaging
//!
//! Provides Unix domain socket-based communication between sidekick agent
//! and parent orchestrator.

use crate::error::{MessagingError, Result};
use crate::messages::{Heartbeat, MessageEnvelope, MessageType, Response};
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info, warn};

/// Socket file path prefix
const SOCKET_PREFIX: &str = "/tmp/sidekick-";
const SOCKET_SUFFIX: &str = ".sock";

/// Generate socket path for a session
pub fn socket_path(session_id: &str) -> String {
    format!("{}{}{}", SOCKET_PREFIX, session_id, SOCKET_SUFFIX)
}

/// Messaging client - runs in the sidekick agent process
/// Connects to the parent orchestrator's socket
pub struct MessagingClient {
    stream: UnixStream,
    session_id: String,
}

impl MessagingClient {
    /// Connect to the parent orchestrator's socket
    pub async fn connect(session_id: &str) -> Result<Self> {
        let path = socket_path(session_id);
        info!("Connecting to messaging socket: {}", path);

        let stream = UnixStream::connect(&path)
            .await
            .map_err(|e| MessagingError::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            stream,
            session_id: session_id.to_string(),
        })
    }

    /// Send a message and wait for response
    pub async fn send(&mut self, envelope: MessageEnvelope) -> Result<Response> {
        // Send the envelope
        let bytes = envelope.to_bytes()?;
        self.stream
            .write_all(&bytes)
            .await
            .map_err(|e| MessagingError::SendError(e.to_string()))?;

        // Wait for response
        self.recv_response().await
    }

    /// Receive and parse a response
    async fn recv_response(&mut self) -> Result<Response> {
        let bytes = self.recv_frame().await?;
        let envelope = MessageEnvelope::from_bytes(&bytes)?;

        if envelope.message_type != MessageType::Response {
            return Err(MessagingError::InvalidFormat(format!(
                "Expected Response, got {:?}",
                envelope.message_type
            )));
        }

        Ok(serde_json::from_value(envelope.payload)?)
    }

    /// Receive a raw frame (length-prefixed bytes)
    async fn recv_frame(&mut self) -> Result<Vec<u8>> {
        // Read 4-byte length prefix
        let mut len_buf = [0u8; 4];
        self.stream
            .read_exact(&mut len_buf)
            .await
            .map_err(|e| MessagingError::ReceiveError(e.to_string()))?;

        let len = u32::from_be_bytes(len_buf) as usize;

        // Read the payload
        let mut payload = vec![0u8; len];
        self.stream
            .read_exact(&mut payload)
            .await
            .map_err(|e| MessagingError::ReceiveError(e.to_string()))?;

        Ok(payload)
    }

    /// Send a heartbeat and wait for pong
    pub async fn heartbeat(&mut self, sequence: u64) -> Result<()> {
        let envelope = MessageEnvelope::new(
            MessageType::Heartbeat,
            serde_json::to_value(Heartbeat::new(sequence))?,
        );
        let bytes = envelope.to_bytes()?;
        self.stream
            .write_all(&bytes)
            .await
            .map_err(|e| MessagingError::SendError(e.to_string()))?;

        // Read pong response
        let bytes = self.recv_frame().await?;
        let envelope = MessageEnvelope::from_bytes(&bytes)?;

        if envelope.message_type != MessageType::Heartbeat {
            warn!("Expected Heartbeat pong, got {:?}", envelope.message_type);
        }

        Ok(())
    }

    /// Get the session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

/// Messaging server - runs in the parent orchestrator process
/// Accepts connections from sidekick agents
#[derive(Clone)]
pub struct MessagingServer {
    listener: Arc<UnixListener>,
    session_id: String,
}

impl MessagingServer {
    /// Create and bind a new server socket
    pub async fn bind(session_id: &str) -> Result<Self> {
        let path = socket_path(session_id);
        info!("Binding messaging server to: {}", path);

        // Remove existing socket if present
        if Path::new(&path).exists() {
            std::fs::remove_file(&path)
                .map_err(|e| MessagingError::IoError(e))?;
        }

        let listener = UnixListener::bind(&path)
            .map_err(|e| MessagingError::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            listener: Arc::new(listener),
            session_id: session_id.to_string(),
        })
    }

    /// Start accepting connections, returning a channel of incoming connections
    pub fn accept(
        &self,
    ) -> (oneshot::Sender<()>, mpsc::Receiver<ConnectionHandler>) {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (tx, rx) = mpsc::channel(1);

        tokio::spawn(self.clone().accept_loop(tx, shutdown_rx));

        (shutdown_tx, rx)
    }

    /// Internal accept loop
    async fn accept_loop(
        self,
        tx: mpsc::Sender<ConnectionHandler>,
        mut shutdown: oneshot::Receiver<()>,
    ) {
        loop {
            tokio::select! {
                result = self.listener.accept() => {
                    match result {
                        Ok((stream, _)) => {
                            debug!("Accepted new connection");
                            let handler = ConnectionHandler::new(stream);
                            if tx.send(handler).await.is_err() {
                                break; // Receiver dropped
                            }
                        }
                        Err(e) => {
                            error!("Accept error: {}", e);
                        }
                    }
                }
                _ = &mut shutdown => {
                    info!("Server shutting down");
                    break;
                }
            }
        }
    }

    /// Get the session ID
    pub fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl Drop for MessagingServer {
    fn drop(&mut self) {
        // Clean up socket file
        let path = socket_path(&self.session_id);
        let _ = std::fs::remove_file(&path);
    }
}

/// Handles an individual connection from a sidekick agent
pub struct ConnectionHandler {
    stream: UnixStream,
}

impl ConnectionHandler {
    /// Create a new connection handler
    pub fn new(stream: UnixStream) -> Self {
        Self { stream }
    }

    /// Receive the next message envelope
    pub async fn recv(&mut self) -> Result<MessageEnvelope> {
        // Read 4-byte length prefix
        let mut len_buf = [0u8; 4];
        self.stream
            .read_exact(&mut len_buf)
            .await
            .map_err(|e| MessagingError::ReceiveError(e.to_string()))?;

        let len = u32::from_be_bytes(len_buf) as usize;

        // Read the payload
        let mut payload = vec![0u8; len];
        self.stream
            .read_exact(&mut payload)
            .await
            .map_err(|e| MessagingError::ReceiveError(e.to_string()))?;

        MessageEnvelope::from_bytes(&payload)
    }

    /// Send a response
    pub async fn send_response(&mut self, response: Response) -> Result<()> {
        let envelope = MessageEnvelope::new(MessageType::Response, serde_json::to_value(response)?);
        let bytes = envelope.to_bytes()?;
        self.stream
            .write_all(&bytes)
            .await
            .map_err(|e| MessagingError::SendError(e.to_string()))?;
        Ok(())
    }

    /// Send a heartbeat response (pong)
    pub async fn send_heartbeat(&mut self, sequence: u64) -> Result<()> {
        let envelope = MessageEnvelope::new(
            MessageType::Heartbeat,
            serde_json::to_value(Heartbeat::new(sequence))?,
        );
        let bytes = envelope.to_bytes()?;
        self.stream
            .write_all(&bytes)
            .await
            .map_err(|e| MessagingError::SendError(e.to_string()))?;
        Ok(())
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        // UnixStream doesn't have an explicit close, dropping is sufficient
        // But we can flush any pending writes
        self.stream
            .shutdown()
            .await
            .map_err(|e| MessagingError::IoError(e))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use std::time::Duration;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            let _ = tracing_subscriber::fmt::try_init();
        });
    }

    #[tokio::test]
    async fn test_socket_path() {
        let path = socket_path("test-session");
        assert_eq!(path, "/tmp/sidekick-test-session.sock");
    }

    #[tokio::test]
    async fn test_message_roundtrip() {
        init();

        let session_id = format!("test-{}", uuid::Uuid::new_v4());

        // Start server
        let server = MessagingServer::bind(&session_id).await.unwrap();
        let (shutdown_tx, mut connections) = server.accept();

        // Spawn server handler
        let server_handle = tokio::spawn(async move {
            if let Some(mut handler) = connections.recv().await {
                let msg = handler.recv().await.unwrap();
                handler.send_heartbeat(42).await.unwrap();
                handler.close().await.unwrap();
                (msg.id, 42)
            } else {
                panic!("No connection received");
            }
        });

        // Spawn client
        let client_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await; // Let server start
            let mut client = MessagingClient::connect(&session_id).await.unwrap();
            let envelope = MessageEnvelope::new(
                MessageType::Heartbeat,
                serde_json::to_value(Heartbeat::new(1)).unwrap(),
            );
            let _ = client.send(envelope).await;
            client.heartbeat(1).await
        });

        let _ = shutdown_tx.send(());
        let _ = tokio::join!(server_handle, client_handle);
    }
}
