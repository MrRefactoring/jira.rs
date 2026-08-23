use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use jira::core::{Client, RetryOptions, with_retry};
use serde_json::json;
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fast(retry_rate_limit: bool) -> RetryOptions {
    RetryOptions {
        max_attempts: 3,
        initial_delay: Duration::from_millis(1),
        backoff_factor: 2.0,
        retry_rate_limit,
    }
}

async fn server_answering(status: u16, headers: &[(&str, &str)]) -> MockServer {
    let server = MockServer::start().await;
    let mut response = ResponseTemplate::new(status).set_body_json(json!({ "message": "nope" }));

    for (name, value) in headers {
        response = response.insert_header(*name, *value);
    }

    Mock::given(method("GET")).respond_with(response).mount(&server).await;

    server
}

#[tokio::test]
async fn retries_a_503() {
    let server = server_answering(503, &[]).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = with_retry(
        || client.get("/rest/api/3/myself").send::<serde_json::Value>(),
        fast(false),
    )
    .await
    .unwrap_err();

    assert!(error.is_server());
    assert_eq!(server.received_requests().await.unwrap().len(), 3);
}

#[tokio::test]
async fn stops_as_soon_as_a_call_succeeds() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .with_priority(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true })))
        .with_priority(2)
        .mount(&server)
        .await;

    let client = Client::builder().host(server.uri()).build().unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);

    let value: serde_json::Value = with_retry(
        || {
            counter.fetch_add(1, Ordering::SeqCst);

            client.get("/rest/api/3/myself").send()
        },
        fast(false),
    )
    .await
    .unwrap();

    assert_eq!(value["ok"], true);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn does_not_retry_a_429_by_default() {
    let server = server_answering(429, &[("retry-after", "1")]).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = with_retry(
        || client.get("/rest/api/3/myself").send::<serde_json::Value>(),
        fast(false),
    )
    .await
    .unwrap_err();

    assert!(error.is_rate_limit());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn retries_a_429_when_asked_honouring_retry_after() {
    let server = server_answering(429, &[("retry-after", "0")]).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = with_retry(
        || client.get("/rest/api/3/myself").send::<serde_json::Value>(),
        fast(true),
    )
    .await
    .unwrap_err();

    assert!(error.is_rate_limit());
    assert_eq!(server.received_requests().await.unwrap().len(), 3);
}

#[tokio::test]
async fn never_retries_a_404() {
    let server = server_answering(404, &[]).await;
    let client = Client::builder().host(server.uri()).build().unwrap();

    let error = with_retry(
        || client.get("/rest/api/3/myself").send::<serde_json::Value>(),
        fast(false),
    )
    .await
    .unwrap_err();

    assert!(error.is_not_found());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn a_client_future_can_be_moved_between_threads() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true })))
        .mount(&server)
        .await;

    let client = Client::builder().host(server.uri()).build().unwrap();
    // The point of the test is that this compiles: `tokio::spawn` demands a `Send` future, and holding a non-`Send`
    // value across an await inside the transport would take that away from every caller.
    let value = tokio::spawn(async move { client.get("/rest/api/3/myself").send::<serde_json::Value>().await })
        .await
        .unwrap()
        .unwrap();

    assert_eq!(value["ok"], true);
}
