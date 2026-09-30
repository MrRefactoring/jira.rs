use std::sync::OnceLock;
use std::time::Duration;

use jira::server::ServerClient;
use jira::{Auth, Client, RetryConfig};

use jira::assets_server::AssetsServerClient;
use jira::service_desk_server::ServiceDeskServerClient;

use super::env::{require_jsm_env, require_server_env};

const RETRY: RetryConfig =
    RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(300), backoff_factor: 2.0 };

const TIMEOUT: Duration = Duration::from_secs(60);

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
            .timeout(TIMEOUT)
            .build()
            .expect("the Data Center credentials describe a usable client")
    })
}

pub fn server() -> &'static ServerClient {
    static SURFACE: OnceLock<ServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| ServerClient::new(server_client().clone()))
}

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
            .timeout(TIMEOUT)
            .build()
            .expect("the Service Management credentials describe a usable client")
    })
}

pub fn assets_server() -> &'static AssetsServerClient {
    static SURFACE: OnceLock<AssetsServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| AssetsServerClient::new(jsm_client().clone()))
}

pub fn jsm_platform() -> &'static ServerClient {
    static SURFACE: OnceLock<ServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| ServerClient::new(jsm_client().clone()))
}

pub fn service_desk_server() -> &'static ServiceDeskServerClient {
    static SURFACE: OnceLock<ServiceDeskServerClient> = OnceLock::new();

    SURFACE.get_or_init(|| ServiceDeskServerClient::new(jsm_client().clone()))
}
