#![cfg(feature = "service-desk-server")]

use jira::Client;
use jira::service_desk_server::{QueueSettings, ServiceDeskServerClient};
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn fixture(body: Value) -> (MockServer, ServiceDeskServerClient) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/servicedeskapi/admin/queues/TEST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(&server)
        .await;
    let client = Client::builder().host(server.uri()).build().unwrap();
    (server, ServiceDeskServerClient::new(client))
}

#[tokio::test]
async fn queue_settings_returns_typed_boolean_settings() {
    let (_server, client) = fixture(json!({
        "queueCount": { "includeQueueCount": true, "useCachedQueueCount": false },
    }))
    .await;
    let settings: QueueSettings = client.queue_settings().get_queue_settings_on_project("TEST").send().await.unwrap();

    assert!(settings.queue_count.include_queue_count);
    assert!(!settings.queue_count.use_cached_queue_count);
}

#[tokio::test]
async fn queue_settings_rejects_missing_and_incorrectly_typed_settings() {
    for body in [
        json!({}),
        json!({ "queueCount": { "includeQueueCount": true } }),
        json!({ "queueCount": { "includeQueueCount": "true", "useCachedQueueCount": false } }),
    ] {
        let (_server, client) = fixture(body).await;
        let error = client.queue_settings().get_queue_settings_on_project("TEST").send().await.unwrap_err();

        assert!(error.is_schema_mismatch());
    }
}
