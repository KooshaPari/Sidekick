//! Message types for sidekick-messaging IPC
//!
//! Defines the core message types for agent-to-parent communication:
//! - Command — request from parent to sidekick
//! - Event — async signal from sidekick to parent
//! - Response — reply from sidekick to parent for a command
//! - Heartbeat — keepalive ping/pong for connection monitoring

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub uuid::Uuid);

impl MessageId {
    /// Generate a new unique message ID
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type of message in the IPC protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Request from parent to sidekick
    Command,
    /// Async signal from sidekick to parent
    Event,
    /// Reply from sidekick to parent
    Response,
    /// Keepalive ping/pong
    Heartbeat,
}

/// Wrapper envelope for all IPC messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    /// Unique message identifier
    pub id: MessageId,
    /// Type of message
    pub message_type: MessageType,
    /// Message payload (JSON)
    pub payload: serde_json::Value,
}

impl MessageEnvelope {
    /// Create a new message envelope
    pub fn new(message_type: MessageType, payload: serde_json::Value) -> Self {
        Self {
            id: MessageId::new(),
            message_type,
            payload,
        }
    }

    /// Serialize the envelope to bytes (4-byte length prefix + JSON)
    pub fn to_bytes(&self) -> crate::error::Result<Vec<u8>> {
        let json = serde_json::to_vec(self)?;
        let len = (json.len() as u32).to_be_bytes();
        let mut bytes = Vec::with_capacity(4 + json.len());
        bytes.extend_from_slice(&len);
        bytes.extend_from_slice(&json);
        Ok(bytes)
    }

    /// Deserialize from bytes (4-byte length prefix + JSON)
    pub fn from_bytes(bytes: &[u8]) -> crate::error::Result<Self> {
        if bytes.len() < 4 {
            return Err(crate::error::MessagingError::InvalidFormat(
                "Message too short".into(),
            ));
        }
        let len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        if bytes.len() < 4 + len {
            return Err(crate::error::MessagingError::InvalidFormat(
                format!("Expected {} bytes, got {}", len, bytes.len() - 4),
            ));
        }
        let json = &bytes[4..4 + len];
        Ok(serde_json::from_slice(json)?)
    }
}

/// Command from parent to sidekick
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Command {
    /// Start a new task
    RunTask { task_id: String, prompt: String },
    /// Abort a running task
    AbortTask { task_id: String },
    /// Update session configuration
    UpdateConfig { settings: serde_json::Value },
    /// Request status
    Status,
}

/// Event from sidekick to parent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    /// Task started
    TaskStarted { task_id: String },
    /// Task progress update
    TaskProgress { task_id: String, progress: f32, message: String },
    /// Task completed
    TaskCompleted { task_id: String, result: serde_json::Value },
    /// Task failed
    TaskFailed { task_id: String, error: String },
    /// Status change
    StatusChanged { status: String },
}

/// Response to a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Original message ID this responds to
    pub in_reply_to: MessageId,
    /// Whether the operation succeeded
    pub success: bool,
    /// Response data or error message
    pub data: serde_json::Value,
    /// Optional error message
    pub error: Option<String>,
}

impl Response {
    /// Create a successful response
    pub fn ok(in_reply_to: MessageId, data: serde_json::Value) -> Self {
        Self {
            in_reply_to,
            success: true,
            data,
            error: None,
        }
    }

    /// Create an error response
    pub fn error(in_reply_to: MessageId, error: impl Into<String>) -> Self {
        Self {
            in_reply_to,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error.into()),
        }
    }
}

/// Heartbeat message for connection monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Timestamp of the heartbeat
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Sequence number
    pub sequence: u64,
}

impl Heartbeat {
    /// Create a new heartbeat
    pub fn new(sequence: u64) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            sequence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_envelope_roundtrip() {
        let envelope = MessageEnvelope::new(
            MessageType::Command,
            serde_json::json!({"test": "data"}),
        );
        let bytes = envelope.to_bytes().unwrap();
        let decoded = MessageEnvelope::from_bytes(&bytes).unwrap();
        assert_eq!(envelope.id, decoded.id);
        assert_eq!(envelope.message_type, decoded.message_type);
    }

    #[test]
    fn test_response_success() {
        let id = MessageId::new();
        let response = Response::ok(id, serde_json::json!({"result": "ok"}));
        assert!(response.success);
        assert!(response.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let id = MessageId::new();
        let response = Response::error(id, "Something went wrong");
        assert!(!response.success);
        assert!(response.error.is_some());
    }

    #[test]
    fn test_heartbeat() {
        let hb = Heartbeat::new(42);
        assert_eq!(hb.sequence, 42);
    }
}
