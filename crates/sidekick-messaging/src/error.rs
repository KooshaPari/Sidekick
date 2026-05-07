//! Error types for sidekick-messaging

use thiserror::Error;

/// Result type alias for messaging operations
pub type Result<T> = std::result::Result<T, MessagingError>;

/// Errors that can occur during messaging operations
#[derive(Error, Debug)]
pub enum MessagingError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Socket I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Channel closed")]
    ChannelClosed,

    #[error("Timeout waiting for response")]
    Timeout,

    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    #[error("Socket path too long (max 108 bytes): {0}")]
    PathTooLong(String),

    #[error("Send error: {0}")]
    SendError(String),

    #[error("Receive error: {0}")]
    ReceiveError(String),
}
