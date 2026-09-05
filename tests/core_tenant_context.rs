use jira::core::{Client, get_tenant_context};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn gateway(body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/gateway/api/graphql"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    server
}

#[tokio::test]
async fn resolves_the_three_names_a_site_has() {
    let server = gateway(json!({
        "data": { "tenantContexts": [{ "cloudId": "cloud-1", "orgId": "org-1", "hostName": "acme.atlassian.net" }] }
    }))
    .await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let context = get_tenant_context(&client).await.unwrap();

    assert_eq!(context.cloud_id, "cloud-1");
    assert_eq!(context.org_id, "org-1");
    assert_eq!(context.host_name, "acme.atlassian.net");
}

#[tokio::test]
async fn asks_about_the_host_the_client_was_built_with() {
    let server = gateway(json!({
        "data": { "tenantContexts": [{ "cloudId": "c", "orgId": "o", "hostName": "h" }] }
    }))
    .await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    get_tenant_context(&client).await.unwrap();

    let request = &server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();

    assert_eq!(body["operationName"], "TenantContext");
    assert_eq!(body["variables"]["hostNames"][0], "127.0.0.1");
}

#[tokio::test]
async fn reports_a_gateway_failure_with_the_status_it_hid_in_the_body() {
    let server = gateway(json!({
        "errors": [{ "message": "Not authorised", "extensions": { "statusCode": 403 } }]
    }))
    .await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = get_tenant_context(&client).await.unwrap_err();

    assert_eq!(error.status(), Some(403));
    assert!(error.to_string().contains("Not authorised"), "{error}");
}

#[tokio::test]
async fn falls_back_to_502_when_the_gateway_names_no_status() {
    let server = gateway(json!({ "errors": [{ "message": "boom" }] })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    assert_eq!(get_tenant_context(&client).await.unwrap_err().status(), Some(502));
}

#[tokio::test]
async fn reports_a_site_atlassian_does_not_know() {
    let server = gateway(json!({ "data": { "tenantContexts": [] } })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = get_tenant_context(&client).await.unwrap_err();

    assert!(error.is_not_found());
}

#[tokio::test]
async fn refuses_a_client_that_carries_no_host_to_ask_about() {
    let client = Client::builder()
        .auth(jira::Auth::oauth2(jira::core::OAuth2Config {
            access_token: Some("token".to_owned()),
            ..jira::core::OAuth2Config::default()
        }))
        .build()
        .unwrap();

    let error = get_tenant_context(&client).await.unwrap_err();

    assert!(error.is_config());
    assert!(error.to_string().contains("carries no host"), "{error}");
}
