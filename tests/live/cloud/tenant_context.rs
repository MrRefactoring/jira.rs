use jira::core::{OAuth2Config, get_tenant_context};
use jira::{Auth, Client};

use crate::harness::{client, require_live_env};

/// The three names Atlassian's platform APIs address a site by.
///
/// Atlassian publishes no REST endpoint for a site's cloud id or organization id, so the GraphQL gateway is the only
/// way the crate can answer the question at all — which makes it worth pinning against the real gateway rather than
/// a mock. The Teams suites depend on it: without a real organization id they cannot address a single call.
fn is_uuid(value: &str) -> bool {
    let groups: Vec<&str> = value.split('-').collect();

    groups.len() == 5
        && [8, 4, 4, 4, 12] == groups.iter().map(|group| group.len()).collect::<Vec<_>>()[..]
        && value.chars().all(|character| character.is_ascii_hexdigit() || character == '-')
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_the_three_names_the_platform_apis_address_this_site_by() {
    let context = get_tenant_context(client()).await.expect("the gateway knows this site");
    let host = require_live_env().host;
    let expected = url::Url::parse(&host).expect("the configured host is a URL");

    assert!(is_uuid(&context.cloud_id), "a cloud id is a UUID: {}", context.cloud_id);
    assert!(is_uuid(&context.org_id), "an organization id is a UUID: {}", context.org_id);
    assert_eq!(Some(context.host_name.as_str()), expected.host_str());
}

/// The gateway and the site's own unauthenticated endpoint agree.
///
/// Two different services answering the same question, which is the only way to tell a correct answer from a
/// well-formed one.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn agrees_with_the_unauthenticated_tenant_endpoint_about_the_cloud_id() {
    let context = get_tenant_context(client()).await.expect("the gateway knows this site");
    let host = require_live_env().host;

    let edge: serde_json::Value = reqwest::get(format!("{host}/_edge/tenant_info"))
        .await
        .expect("the site serves its tenant info")
        .json()
        .await
        .expect("the tenant info is JSON");

    assert_eq!(edge["cloudId"].as_str(), Some(context.cloud_id.as_str()));
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_the_organization_which_is_a_level_above_the_site() {
    let context = get_tenant_context(client()).await.expect("the gateway knows this site");

    assert_ne!(context.org_id, context.cloud_id, "a site and its organization are not the same thing");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_client_with_no_host_instead_of_asking_the_wrong_gateway() {
    let hostless = Client::builder()
        .auth(Auth::oauth2(OAuth2Config {
            access_token: Some("not-used".to_owned()),
            cloud_id: Some("not-used".to_owned()),
            ..OAuth2Config::default()
        }))
        .build()
        .expect("a 3LO client needs no host");

    let error = get_tenant_context(&hostless).await.expect_err("there is no site to ask about");

    assert!(error.is_config(), "{error}");
}
