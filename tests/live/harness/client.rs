use std::sync::OnceLock;
use std::time::Duration;

use jira::admin::AdminClient;
use jira::agile::AgileClient;
use jira::cloud::CloudClient;
use jira::service_desk::ServiceDeskClient;
use jira::teams::TeamsClient;
use jira::user_management::UserManagementClient;
use jira::{Auth, Client, RetryConfig};

use super::env::require_live_env;

/// Rides out the occasional transient reset or gateway error Jira Cloud throws, without masking a real 4xx.
const RETRY: RetryConfig = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis(300),
    backoff_factor: 2.0,
};

/// The one transport every surface is built from.
///
/// Deliberately shared: two clients would mean two auth states, which under OAuth 2.0 is a live bug rather than
/// waste. That every suite works off this instance is the proof the sharing works.
pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let env = require_live_env();

        Client::builder()
            .host(env.host)
            .auth(Auth::api_token(env.email, env.api_token))
            .retry(RETRY)
            .build()
            .expect("the live credentials describe a usable client")
    })
}

macro_rules! surface {
    ($(#[$meta:meta])* $name:ident -> $type:ty) => {
        $(#[$meta])*
        pub fn $name() -> &'static $type {
            static SURFACE: OnceLock<$type> = OnceLock::new();

            SURFACE.get_or_init(|| <$type>::new(client().clone()))
        }
    };
}

surface!(
    /// The Jira Cloud platform surface.
    cloud -> CloudClient
);
surface!(
    /// The Jira Software (Agile) surface.
    agile -> AgileClient
);
surface!(
    /// The Jira Service Management surface.
    service_desk -> ServiceDeskClient
);
surface!(
    /// The Teams surface, which answers on the organization rather than on the site.
    teams -> TeamsClient
);
surface!(
    /// The organization administration surface.
    admin_surface -> AdminClient
);
surface!(
    /// The user management surface.
    user_management -> UserManagementClient
);

/// The organization the site belongs to.
///
/// Read from the environment when it was pinned there, and asked of the site otherwise — a new tenant needs no secret
/// added anywhere.
pub async fn org_id() -> String {
    static ORG_ID: tokio::sync::OnceCell<String> = tokio::sync::OnceCell::const_new();

    ORG_ID
        .get_or_init(|| async {
            if let Some(pinned) = require_live_env().org_id {
                return pinned;
            }

            jira::core::get_tenant_context(client())
                .await
                .expect("the site answers with its tenant context")
                .org_id
        })
        .await
        .clone()
}

/// A client authenticated with the organization API key, for the surfaces a site token cannot reach.
pub fn admin_key_client() -> Client {
    let env = require_live_env();
    let key = env.admin_api_key.expect("an organization API key is configured");

    Client::builder()
        .host("https://api.atlassian.com")
        .auth(Auth::bearer(key))
        .retry(RETRY)
        .build()
        .expect("the organization key describes a usable client")
}
