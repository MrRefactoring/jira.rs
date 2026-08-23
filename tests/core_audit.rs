#![cfg(feature = "audit")]

use jira::core::Client;
use jira::core::audit::{SchemaDrift, collected, reset};
use serde::Deserialize;
use serde_json::json;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

jira::open_enum! {
    pub enum ProjectTypeKey {
        Software => "software",
        Business => "business",
    }
}

#[derive(Debug, Deserialize)]
struct Project {
    #[serde(rename = "projectTypeKey")]
    project_type_key: ProjectTypeKey,
}

async fn answering(body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET")).respond_with(ResponseTemplate::new(200).set_body_json(body)).mount(&server).await;

    server
}

/// The collection is process-wide, so the cases take it in turn rather than racing over it.
///
/// An async lock, not a `std` one: these cases await between taking it and releasing it.
async fn guard() -> tokio::sync::MutexGuard<'static, ()> {
    static LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    LOCK.lock().await
}

#[tokio::test]
async fn records_a_key_the_specification_never_described() {
    let _guard = guard().await;

    reset();

    let server = answering(json!({ "projectTypeKey": "software", "somethingNew": true })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let _: Project = client.get("/rest/api/3/project/X").send().await.unwrap();

    let drift = collected();

    assert!(
        drift.iter().any(|entry| matches!(
            entry,
            SchemaDrift::UndocumentedKeys { path, .. } if path == "somethingNew"
        )),
        "{drift:?}",
    );
}

#[tokio::test]
async fn records_a_value_an_open_enum_had_to_grow_for() {
    let _guard = guard().await;

    reset();

    let server = answering(json!({ "projectTypeKey": "product_discovery" })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let project: Project = client.get("/rest/api/3/project/X").send().await.unwrap();

    assert_eq!(project.project_type_key.as_str(), "product_discovery");

    let drift = collected();

    assert!(
        drift.iter().any(|entry| matches!(
            entry,
            SchemaDrift::UndocumentedValue { value, .. } if value == "product_discovery"
        )),
        "{drift:?}",
    );
}

#[tokio::test]
async fn records_nothing_for_a_response_the_types_describe_completely() {
    let _guard = guard().await;

    reset();

    let server = answering(json!({ "projectTypeKey": "business" })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let _: Project = client.get("/rest/api/3/project/X").send().await.unwrap();

    assert!(collected().is_empty(), "{:?}", collected());
}

#[tokio::test]
async fn says_each_finding_once_however_many_responses_repeat_it() {
    let _guard = guard().await;

    reset();

    let server = answering(json!({ "projectTypeKey": "software", "somethingNew": true })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    for _ in 0..3 {
        let _: Project = client.get("/rest/api/3/project/X").send().await.unwrap();
    }

    assert_eq!(collected().len(), 1, "{:?}", collected());
}

#[tokio::test]
async fn forgets_what_it_collected_when_asked() {
    let _guard = guard().await;

    reset();

    let server = answering(json!({ "projectTypeKey": "grown" })).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let _: Project = client.get("/rest/api/3/project/X").send().await.unwrap();

    assert!(!collected().is_empty());

    reset();

    assert!(collected().is_empty());
}
