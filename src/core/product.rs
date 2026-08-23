pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The Atlassian gateway slug, as in `https://api.atlassian.com/ex/<slug>/{cloudId}`.
pub const GATEWAY_SLUG: &str = "jira";

/// Product-specific advice appended to a scope-mismatch 401.
pub const SCOPE_HINT: &str = "Jira scopes are granted per operation rather than per API version — the scope the \
failing operation names in its API documentation is the one to add.";

/// Sent as `User-Agent` on every request, so Atlassian's logs name the client.
pub const USER_AGENT: &str = concat!("jira-rs/", env!("CARGO_PKG_VERSION"));
