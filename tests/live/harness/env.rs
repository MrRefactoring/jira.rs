use std::sync::OnceLock;

/// The credentials the live suites run against.
#[derive(Debug, Clone)]
pub struct LiveEnv {
    /// Site base URL, e.g. `https://your-site.atlassian.net` — no trailing slash, no API path.
    pub host: String,
    pub email: String,
    pub api_token: String,
    /// The organization the site belongs to, when it was pinned rather than resolved.
    pub org_id: Option<String>,
    /// An organization API key, for the surfaces that answer on `api.atlassian.com` rather than on a site.
    ///
    /// Optional: CI has no such key, and the suites that need one stand down visibly rather than failing. A site API
    /// token does not substitute — those APIs answer 401 to one.
    pub admin_api_key: Option<String>,
}

/// A Data Center instance the `server` suites run against, brought up by `cargo xtask jira-dc up`.
#[derive(Debug, Clone)]
pub struct ServerEnv {
    pub host: String,
    /// A personal access token, where the instance is new enough to insist on one.
    pub pat: Option<String>,
    pub username: String,
    pub password: String,
}

fn load_dotenv() {
    static LOADED: OnceLock<()> = OnceLock::new();

    LOADED.get_or_init(|| {
        // Absent in CI, where the credentials arrive as real environment variables.
        let _ = dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    });
}

fn first_set(names: &[&str]) -> Option<String> {
    load_dotenv();

    names.iter().filter_map(|name| std::env::var(name).ok()).find(|value| !value.trim().is_empty())
}

/// Whether the credentials the Cloud suites need are present.
pub fn has_live_env() -> bool {
    first_set(&["JIRA_BASE_URL", "HOST"]).is_some() && first_set(&["JIRA_EMAIL", "EMAIL"]).is_some()
}

/// Whether an organization API key is configured, which the administration suites need and CI does not have.
pub fn has_admin_env() -> bool {
    first_set(&["JIRA_ADMIN_API_KEY"]).is_some()
}

/// Whether a Data Center instance is reachable, which the `server` suites need.
pub fn has_server_env() -> bool {
    first_set(&["JIRA_SERVER_BASE_URL"]).is_some()
}

/// Whether a Service Management Data Center instance is reachable, which the `jsm` suites need.
pub fn has_jsm_env() -> bool {
    first_set(&["JSM_SERVER_BASE_URL"]).is_some()
}

/// The Service Management Data Center instance, or a single actionable failure.
///
/// A separate rig from the Jira one, on its own port. Both cannot run at once on a machine with less memory than the
/// two of them want, so the suites that need this one are run on their own.
pub fn require_jsm_env() -> ServerEnv {
    let host = first_set(&["JSM_SERVER_BASE_URL"]).map(|host| host.trim_end_matches('/').to_owned());

    match host {
        Some(host) => ServerEnv {
            host,
            pat: first_set(&["JSM_SERVER_PAT"]),
            username: first_set(&["JSM_SERVER_USERNAME"]).unwrap_or_else(|| "admin".to_owned()),
            password: first_set(&["JSM_SERVER_PASSWORD"]).unwrap_or_else(|| "admin123".to_owned()),
        },
        None => panic!(
            "The Service Management Data Center suites need JSM_SERVER_BASE_URL. Bring an instance up with \
`cargo xtask jsm-dc up` — and take the Jira rig down first, they do not fit side by side."
        ),
    }
}

/// The credentials, or a single actionable failure naming what is missing.
pub fn require_live_env() -> LiveEnv {
    let host = first_set(&["JIRA_BASE_URL", "HOST"]).map(|host| host.trim_end_matches('/').to_owned());
    let email = first_set(&["JIRA_EMAIL", "EMAIL"]);
    let api_token = first_set(&["JIRA_API_TOKEN", "API_TOKEN"]);

    match (host, email, api_token) {
        (Some(host), Some(email), Some(api_token)) => LiveEnv {
            host,
            email,
            api_token,
            org_id: first_set(&["JIRA_ORG_ID"]),
            admin_api_key: first_set(&["JIRA_ADMIN_API_KEY"]),
        },
        _ => panic!(
            "Live tests need JIRA_BASE_URL, JIRA_EMAIL and JIRA_API_TOKEN in the crate-root .env.\n\
JIRA_BASE_URL is the bare site URL (https://your-site.atlassian.net) — the suites append the API paths."
        ),
    }
}

/// The Data Center instance, or a single actionable failure.
pub fn require_server_env() -> ServerEnv {
    let host = first_set(&["JIRA_SERVER_BASE_URL"]).map(|host| host.trim_end_matches('/').to_owned());

    match host {
        Some(host) => ServerEnv {
            host,
            pat: first_set(&["JIRA_SERVER_PAT"]),
            username: first_set(&["JIRA_SERVER_USERNAME"]).unwrap_or_else(|| "admin".to_owned()),
            password: first_set(&["JIRA_SERVER_PASSWORD"]).unwrap_or_else(|| "admin123".to_owned()),
        },
        None => panic!(
            "The Data Center suites need JIRA_SERVER_BASE_URL. Bring an instance up with `cargo xtask jira-dc up`."
        ),
    }
}
