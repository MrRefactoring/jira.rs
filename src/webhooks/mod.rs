//! The webhooks a Jira site delivers: what it sends, and how to tell it really sent it.
//!
//! Nothing here calls Jira. A webhook arrives at a server of yours, and this module types what arrives and answers
//! the one question a receiver cannot answer for itself — whether the body was signed by the site or posted by
//! whoever found the URL.
//!
//! ```no_run
//! use jira::webhooks::{WebhookEvent, WebhookPayload, verify_signature};
//!
//! # fn handle(body: &[u8], signature: Option<&str>, secret: &str) -> jira::Result<()> {
//! if !verify_signature(body, secret, signature)? {
//!     return Ok(());
//! }
//!
//! let payload: WebhookPayload = serde_json::from_slice(body).unwrap();
//!
//! match payload.webhook_event {
//!     Some(WebhookEvent::JiraIssueCreated) => { /* payload.issue */ }
//!     Some(WebhookEvent::SprintStarted) => { /* payload.sprint */ }
//!     _ => {}
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Registering a webhook is a different thing and belongs to the API surfaces: `cloud::Webhooks` for a Connect app's
//! dynamic registrations, `server::Webhooks` for a Data Center instance.

pub mod events;
pub mod headers;
pub mod payloads;
pub mod verify;

pub use events::WebhookEvent;
pub use headers::{WebhookFlow, WebhookHeaders};
pub use payloads::WebhookPayload;
pub use verify::verify_signature;
