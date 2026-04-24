//! Multi-provider messaging adapter (iMessage, SMS, Email).
//!
//! Provides unified interface to messaging services via agent-imessage MCP skill.
//! Source: external agent-imessage MCP server (claudeai-proxy)
//! Reference: `mcp__agent-imessage__*` tool suite in Claude Code

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported messaging providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageProvider {
    IMessage,
    SMS,
    Email,
}

impl fmt::Display for MessageProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IMessage => write!(f, "iMessage"),
            Self::SMS => write!(f, "SMS"),
            Self::Email => write!(f, "Email"),
        }
    }
}

/// Message metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub recipient: String,
    pub body: String,
    pub provider: MessageProvider,
}

impl Message {
    /// Create a new message.
    pub fn new(recipient: impl Into<String>, body: impl Into<String>, provider: MessageProvider) -> Self {
        Self {
            recipient: recipient.into(),
            body: body.into(),
            provider,
        }
    }
}

/// Messaging adapter trait for pluggable provider implementations.
pub trait MessagingAdapter: Send + Sync {
    /// Send a message via the provider.
    fn send(&self, message: &Message) -> Result<String, MessagingError>;

    /// Check if recipient has the provider available.
    fn is_available(&self, recipient: &str) -> Result<bool, MessagingError>;
}

/// Messaging errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "error_type")]
pub enum MessagingError {
    ProviderNotAvailable { provider: String },
    InvalidRecipient { recipient: String },
    SendFailed { reason: String },
    ConfigurationError { reason: String },
}

impl fmt::Display for MessagingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProviderNotAvailable { provider } => write!(f, "Provider not available: {}", provider),
            Self::InvalidRecipient { recipient } => write!(f, "Invalid recipient: {}", recipient),
            Self::SendFailed { reason } => write!(f, "Send failed: {}", reason),
            Self::ConfigurationError { reason } => write!(f, "Configuration error: {}", reason),
        }
    }
}

impl std::error::Error for MessagingError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new("user@example.com", "Hello", MessageProvider::Email);
        assert_eq!(msg.recipient, "user@example.com");
        assert_eq!(msg.body, "Hello");
        assert_eq!(msg.provider, MessageProvider::Email);
    }

    #[test]
    fn test_provider_display() {
        assert_eq!(MessageProvider::IMessage.to_string(), "iMessage");
        assert_eq!(MessageProvider::SMS.to_string(), "SMS");
        assert_eq!(MessageProvider::Email.to_string(), "Email");
    }
}
