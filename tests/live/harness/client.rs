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

const RETRY: RetryConfig =
    RetryConfig { max_attempts: 3, initial_delay: Duration::from_millis(300), backoff_factor: 2.0 };

const TIMEOUT: Duration = Duration::from_secs(60);

pub fn client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let env = require_live_env();

        Client::builder()
            .host(env.host)
            .auth(Auth::api_token(env.email, env.api_token))
            .retry(RETRY)
            .timeout(TIMEOUT)
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
    cloud -> CloudClient
);
surface!(
    agile -> AgileClient
);
surface!(
    service_desk -> ServiceDeskClient
);
surface!(
    teams -> TeamsClient
);
surface!(
    admin_surface -> AdminClient
);
surface!(
    user_management -> UserManagementClient
);

pub async fn org_id() -> String {
    static ORG_ID: tokio::sync::OnceCell<String> = tokio::sync::OnceCell::const_new();

    ORG_ID
        .get_or_init(|| async {
            if let Some(pinned) = require_live_env().org_id {
                return pinned;
            }

            jira::core::get_tenant_context(client()).await.expect("the site answers with its tenant context").org_id
        })
        .await
        .clone()
}

pub async fn site_id() -> String {
    static SITE_ID: tokio::sync::OnceCell<String> = tokio::sync::OnceCell::const_new();

    SITE_ID
        .get_or_init(|| async {
            jira::core::get_tenant_context(client()).await.expect("the site answers with its tenant context").cloud_id
        })
        .await
        .clone()
}

pub fn admin_key_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();

    CLIENT.get_or_init(|| {
        let env = require_live_env();
        let key = env.admin_api_key.expect("an organization API key is configured");

        Client::builder()
            .host("https://api.atlassian.com")
            .auth(Auth::bearer(key))
            .retry(RETRY)
            .timeout(TIMEOUT)
            .build()
            .expect("the organization key describes a usable client")
    })
}
