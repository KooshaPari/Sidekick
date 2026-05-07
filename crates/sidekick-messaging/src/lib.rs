//! # sidekick-messaging
//!
//! Sidekick agent-to-parent IPC and multi-provider user notification.
//!
//! ## FR-MSG: Agent-to-Parent IPC
//!
//! Handles bidirectional communication between the sidekick agent and its parent
//! orchestrator via Unix domain sockets.
//!
//! **IPC Protocol:**
//! - Transport: Unix domain stream socket (`/tmp/sidekick-<session>.sock`)
//! - Framing: 4-byte big-endian length prefix + JSON payload
//! - Serialization: JSON via `serde_json`
//!
//! **Message Types:**
//! - [`Command`] — request from parent to sidekick (e.g. "run task", "abort")
//! - [`Event`] — asynchronous signal from sidekick to parent (e.g. "task started", "progress")
//! - [`Response`] — reply from sidekick to parent for a command
//! - [`Heartbeat`] — keepalive ping/pong to detect connection loss
//!
//! **Architecture:**
//! - [`MessagingClient`] — runs in the sidekick agent process; connects to parent socket
//! - [`MessagingServer`] — runs in the parent process; accepts sidekick connections
//!
//! ## User Notifications (legacy)
//!
//! Multi-provider messaging adapter (iMessage, SMS, Email) via `agent-imessage` MCP skill.

pub mod error;
pub mod ipc;
pub mod messages;

pub use error::{MessagingError, Result};
pub use ipc::{MessagingClient, MessagingServer};
pub use messages::{Command, Event, Heartbeat, MessageEnvelope, MessageId, MessageType, Response};
