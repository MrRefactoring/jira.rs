use std::sync::OnceLock;
use std::time::Duration;

use jira::server::ServerClient;
use jira::{Auth, Client, RetryConfig};

use jira::assets_server::AssetsServerClient;
use jira::service_desk_server::ServiceDeskServerClient;

use super::env::{require_jsm_env, require_server_env};

const RETRY: RetryConfig =
    RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(300), backoff_factor: 2.0 };

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

/// The transport the Service Management Data Center suites use.
fn jsm_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let env = require_jsm_env();
        let auth = match env.pat {
            Some(token) => Auth::bearer(token),
            None => Auth::password(env.username, env.password),
        };

        Client::builder()
            .host(env.host)
            .auth(auth)
            .retry(RETRY)
            .build()
            .expect("the Service Management credentials describe a usable client")
    })
}

/// Assets, as a self-hosted instance serves it.
pub fn assets_server() -> &'static AssetsServerClient {
    static SURFACE: OnceLock<AssetsServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| AssetsServerClient::new(jsm_client().clone()))
}

/// Service Management, as a self-hosted instance serves it.
pub fn service_desk_server() -> &'static ServiceDeskServerClient {
    static SURFACE: OnceLock<ServiceDeskServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| ServiceDeskServerClient::new(jsm_client().clone()))
}
