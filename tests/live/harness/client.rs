use std::sync::OnceLock;
use std::time::Duration;

use jira::admin::AdminClient;
use jira::agile::AgileClient;
use jira::cloud::ActorsMap;
use jira::cloud::CloudClient;
use jira::service_desk::ServiceDeskClient;
use jira::teams::TeamsClient;
use jira::user_management::UserManagementClient;
use jira::{Auth, Client, RetryConfig};

use super::env::require_live_env;
use super::fixtures::TEST_PROJECT_KEY;

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

pub async fn ensure_access_role() {
    static ACCESS_ROLE: tokio::sync::OnceCell<()> = tokio::sync::OnceCell::const_new();

    ACCESS_ROLE
        .get_or_init(|| async {
            const ROLE: &str = "Administrators";

            cloud().projects().get_project(TEST_PROJECT_KEY).send().await.unwrap_or_else(|error| {
                panic!("the test project {TEST_PROJECT_KEY} must exist for the {ROLE} role: {error}")
            });

            let roles =
                cloud().project_roles().get_project_roles(TEST_PROJECT_KEY).send().await.unwrap_or_else(|error| {
                    panic!("the test project {TEST_PROJECT_KEY} must list its {ROLE} role: {error}")
                });
            let id = roles
                .get(ROLE)
                .and_then(|url| url.rsplit('/').next())
                .and_then(|id| id.parse::<i64>().ok())
                .unwrap_or_else(|| panic!("the test project {TEST_PROJECT_KEY} has no {ROLE} role with a numeric id"));
            let account_id = cloud()
                .myself()
                .get_current_user()
                .send()
                .await
                .expect("the site knows the test account")
                .account_id
                .expect("the test account carries an account id");
            let role = cloud()
                .project_roles()
                .get_project_role(TEST_PROJECT_KEY, id)
                .send()
                .await
                .unwrap_or_else(|error| panic!("the {ROLE} role of {TEST_PROJECT_KEY} must read back: {error}"));

            if role.actors.unwrap_or_default().iter().any(|actor| {
                actor.actor_user.as_ref().and_then(|user| user.account_id.as_deref()) == Some(account_id.as_str())
            }) {
                return;
            }

            cloud()
                .project_role_actors()
                .add_actor_users(
                    TEST_PROJECT_KEY,
                    id,
                    ActorsMap { user: Some(vec![account_id]), ..ActorsMap::default() },
                )
                .send()
                .await
                .unwrap_or_else(|error| {
                    panic!("the test account must join the {ROLE} role of {TEST_PROJECT_KEY}: {error}")
                });
        })
        .await;
}

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
