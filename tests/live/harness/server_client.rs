use std::sync::OnceLock;
use std::time::Duration;

use jira::server::ServerClient;
use jira::{Auth, Client, RetryConfig};

use super::env::require_server_env;

const RETRY: RetryConfig = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis(300),
    backoff_factor: 2.0,
};

/// The transport the Data Center suites use.
///
/// A personal access token where the instance was given one, and the administrator's password otherwise: Jira 10.3
/// still accepts basic authentication, and the rig's throwaway instance has no other credential to offer.
pub fn server_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let env = require_server_env();
        let auth = match env.pat {
            Some(token) => Auth::bearer(token),
            None => Auth::password(env.username, env.password),
        };

        Client::builder()
            .host(env.host)
            .auth(auth)
            .retry(RETRY)
            .build()
            .expect("the Data Center credentials describe a usable client")
    })
}

/// The Jira Data Center surface — platform and Agile in one.
pub fn server() -> &'static ServerClient {
    static SURFACE: OnceLock<ServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| ServerClient::new(server_client().clone()))
}
