use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct LiveEnv {
    pub host: String,
    pub email: String,
    pub api_token: String,
    pub org_id: Option<String>,
    pub admin_api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ServerEnv {
    pub host: String,
    pub pat: Option<String>,
    pub username: String,
    pub password: String,
}

fn load_dotenv() {
    static LOADED: OnceLock<()> = OnceLock::new();

    LOADED.get_or_init(|| {
        let _ = dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/.env"));
    });
}

fn first_set(names: &[&str]) -> Option<String> {
    load_dotenv();

    names.iter().filter_map(|name| std::env::var(name).ok()).find(|value| !value.trim().is_empty())
}

pub fn has_admin_env() -> bool {
    first_set(&["JIRA_ADMIN_API_KEY"]).is_some()
}

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
