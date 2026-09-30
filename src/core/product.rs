/// This crate's version, as in `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const GATEWAY_SLUG: &str = "jira";

pub const SCOPE_HINT: &str = "Jira scopes are granted per operation rather than per API version — the scope the \
failing operation names in its API documentation is the one to add.";

/// Sent as `User-Agent` on every request, so Atlassian's logs name the client.
pub const USER_AGENT: &str = concat!("jira-rs/", env!("CARGO_PKG_VERSION"));
