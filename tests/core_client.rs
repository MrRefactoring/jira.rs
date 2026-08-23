use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use jira::core::{Attachment, Auth, Body, Client, MultipartBody, RetryConfig};
use serde::Deserialize;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

#[derive(Debug, Deserialize)]
struct Myself {
    #[serde(rename = "displayName")]
    display_name: String,
}

async fn server_answering(body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/rest/api/3/myself"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(&server)
        .await;

    server
}

fn client_for(server: &MockServer, auth: Option<Auth>) -> Client {
    let mut builder = Client::builder().host(server.uri());

    if let Some(auth) = auth {
        builder = builder.auth(auth);
    }

    builder.build().expect("the client is configured")
}

async fn first_request(server: &MockServer) -> Request {
    server
        .received_requests()
        .await
        .expect("the server recorded its requests")
        .remove(0)
}

fn header_of(request: &Request, name: &str) -> Option<String> {
    request
        .headers
        .get(name)
        .map(|value| value.to_str().unwrap_or_default().to_owned())
}

// ---------------------------------------------------------------- auth headers

#[tokio::test]
async fn base64_encodes_email_and_token_for_basic_auth() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = client_for(&server, Some(Auth::api_token("you@example.com", "TOKEN")));

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    // `you@example.com:TOKEN`
    assert_eq!(
        header_of(&first_request(&server).await, "authorization").as_deref(),
        Some("Basic eW91QGV4YW1wbGUuY29tOlRPS0VO"),
    );
}

#[tokio::test]
async fn base64_encodes_username_and_password_for_data_center_basic_auth() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = client_for(&server, Some(Auth::password("ada", "hunter2")));

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(
        header_of(&first_request(&server).await, "authorization").as_deref(),
        Some("Basic YWRhOmh1bnRlcjI="),
    );
}

#[tokio::test]
async fn encodes_a_credential_that_is_not_ascii() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = client_for(&server, Some(Auth::password("ada", "пароль")));

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    let expected = format!("Basic {}", {
        use std::fmt::Write as _;
        let _ = String::new();
        // Computed the same way any other base64 implementation would, from the UTF-8 bytes.
        let mut encoded = String::new();
        let input = "ada:пароль".as_bytes();
        const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        for chunk in input.chunks(3) {
            let bytes = [
                chunk[0],
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
            ];
            let triple = (u32::from(bytes[0]) << 16) | (u32::from(bytes[1]) << 8) | u32::from(bytes[2]);
            for index in 0..4 {
                if index <= chunk.len() {
                    let _ = write!(
                        encoded,
                        "{}",
                        char::from(ALPHABET[((triple >> (18 - index * 6)) & 0x3F) as usize])
                    );
                } else {
                    encoded.push('=');
                }
            }
        }
        encoded
    });

    assert_eq!(
        header_of(&first_request(&server).await, "authorization"),
        Some(expected)
    );
}

#[tokio::test]
async fn sends_a_static_bearer_token() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = client_for(&server, Some(Auth::bearer("PAT")));

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(
        header_of(&first_request(&server).await, "authorization").as_deref(),
        Some("Bearer PAT")
    );
}

#[tokio::test]
async fn resolves_a_bearer_token_provider_per_request() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let calls = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&calls);
    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::BearerProvider(Arc::new(move || {
            let counter = Arc::clone(&counter);

            async move { Ok(format!("minted-{}", counter.fetch_add(1, Ordering::SeqCst))) }
        })))
        .build()
        .unwrap();

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();
    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    let requests = server.received_requests().await.unwrap();

    assert_eq!(
        header_of(&requests[0], "authorization").as_deref(),
        Some("Bearer minted-0")
    );
    assert_eq!(
        header_of(&requests[1], "authorization").as_deref(),
        Some("Bearer minted-1")
    );
}

#[tokio::test]
async fn sends_no_authorization_when_unauthenticated() {
    let server = server_answering(json!({ "displayName": "Anonymous" })).await;
    let client = client_for(&server, None);

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(header_of(&first_request(&server).await, "authorization"), None);
}

#[tokio::test]
async fn names_itself_in_the_user_agent() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = client_for(&server, None);

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    let agent = header_of(&first_request(&server).await, "user-agent").unwrap_or_default();

    assert!(agent.starts_with("jira-rs/"), "{agent}");
}

#[tokio::test]
async fn re_derives_auth_once_on_401_and_retries() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/rest/api/3/myself"))
        .and(header("authorization", "Bearer stale"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "message": "Client must be authenticated" })))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/rest/api/3/myself"))
        .and(header("authorization", "Bearer fresh"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::bearer("stale"))
        .get_auth_on_401(|| async { Ok(Auth::bearer("fresh")) })
        .build()
        .unwrap();

    let myself: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(myself.display_name, "Ada");
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn surfaces_the_401_when_no_refresh_hook_is_configured() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "message": "Client must be authenticated" })))
        .mount(&server)
        .await;

    let client = client_for(&server, Some(Auth::bearer("stale")));
    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn does_not_loop_when_the_fresh_credentials_are_refused_as_well() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({ "message": "no" })))
        .mount(&server)
        .await;

    let attempts = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&attempts);
    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::bearer("stale"))
        .get_auth_on_401(move || {
            let counter = Arc::clone(&counter);

            async move {
                counter.fetch_add(1, Ordering::SeqCst);

                Ok(Auth::bearer("also-stale"))
            }
        })
        .build()
        .unwrap();

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());
    assert_eq!(
        attempts.load(Ordering::SeqCst),
        1,
        "the hook is given one attempt, not a loop"
    );
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

// ---------------------------------------------------------------- errors

#[tokio::test]
async fn reports_a_failure_carrying_status_and_parsed_body() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(400).set_body_json(json!({ "errorMessages": ["Field 'foo' cannot be set"] })),
        )
        .mount(&server)
        .await;

    let error = client_for(&server, None)
        .get("/rest/api/3/issue/X")
        .send::<Myself>()
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(400));
    assert_eq!(error.body().unwrap()["errorMessages"][0], "Field 'foo' cannot be set");
}

#[tokio::test]
async fn keeps_a_non_json_error_body_as_raw_text() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503).set_body_string("upstream is down"))
        .mount(&server)
        .await;

    let error = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send::<Myself>()
        .await
        .unwrap_err();

    assert!(error.is_server());
    assert_eq!(error.body().unwrap(), &json!("upstream is down"));
}

// ---------------------------------------------------------------- responses

#[tokio::test]
async fn returns_nothing_for_a_204() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client_for(&server, None)
        .delete("/rest/api/3/issue/X")
        .send_empty()
        .await
        .unwrap();
}

#[tokio::test]
async fn deserializes_the_body_into_the_declared_type() {
    let server = server_answering(json!({ "displayName": "Ada", "accountId": "5b10" })).await;

    let myself: Myself = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send()
        .await
        .unwrap();

    assert_eq!(myself.display_name, "Ada");
}

#[tokio::test]
async fn ignores_a_field_atlassian_added_since() {
    let server = server_answering(json!({ "displayName": "Ada", "somethingNew": { "nested": true } })).await;

    let myself: Myself = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send()
        .await
        .unwrap();

    assert_eq!(myself.display_name, "Ada");
}

#[tokio::test]
async fn reports_drift_by_field_rather_than_by_offset() {
    let server = server_answering(json!({ "displayName": 42 })).await;

    let error = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send::<Myself>()
        .await
        .unwrap_err();
    let report = error.schema_report().expect("a schema mismatch carries a report");

    assert!(error.is_schema_mismatch());
    assert_eq!(report.endpoint, "GET /rest/api/3/myself");
    assert_eq!(report.issues[0].path, "displayName");
    assert_eq!(report.issues[0].received, "number");
}

#[tokio::test]
async fn names_the_missing_field_when_the_response_leaves_one_out() {
    let server = server_answering(json!({ "accountId": "5b10" })).await;

    let error = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send::<Myself>()
        .await
        .unwrap_err();
    let report = error.schema_report().unwrap();

    assert!(
        report.issues[0].expected.contains("displayName"),
        "{:?}",
        report.issues[0]
    );
}

#[tokio::test]
async fn rejects_a_non_json_response_where_a_type_was_expected() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("<html>login</html>", "text/html"))
        .mount(&server)
        .await;

    let error = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send::<Myself>()
        .await
        .unwrap_err();
    let report = error.schema_report().unwrap();

    assert_eq!(report.issues[0].expected, "application/json");
    assert_eq!(report.issues[0].received, "text/html");
}

#[tokio::test]
async fn falls_back_to_raw_text_when_jira_mislabels_plain_text_as_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("ok", "application/json"))
        .mount(&server)
        .await;

    let answer: String = client_for(&server, None).get("/rest/api/3/ping").send().await.unwrap();

    assert_eq!(answer, "ok");
}

#[tokio::test]
async fn returns_the_raw_bytes_of_a_download() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0x89, 0x50, 0x4E, 0x47], "image/png"))
        .mount(&server)
        .await;

    let bytes = client_for(&server, None)
        .get("/rest/api/3/attachment/1")
        .send_bytes()
        .await
        .unwrap();

    assert_eq!(bytes.as_ref(), &[0x89, 0x50, 0x4E, 0x47]);
}

#[tokio::test]
async fn hands_back_the_json_body_unmodelled() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;

    let value = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send_raw()
        .await
        .unwrap();

    assert_eq!(value["displayName"], "Ada");
}

// ---------------------------------------------------------------- retry

#[tokio::test]
async fn retry_is_off_by_default() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let error = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send::<Myself>()
        .await
        .unwrap_err();

    assert!(error.is_server());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn retries_a_503_up_to_max_attempts() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .retry(RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            backoff_factor: 2.0,
        })
        .build()
        .unwrap();

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_server());
    assert_eq!(server.received_requests().await.unwrap().len(), 3);
}

#[tokio::test]
async fn stops_retrying_as_soon_as_a_call_succeeds() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(503))
        .up_to_n_times(1)
        .with_priority(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .with_priority(2)
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .retry(RetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(1),
            backoff_factor: 2.0,
        })
        .build()
        .unwrap();

    let myself: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(myself.display_name, "Ada");
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[tokio::test]
async fn never_retries_a_429_because_rate_limiting_is_not_a_transport_failure_to_paper_over() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "30"))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .retry(RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            backoff_factor: 2.0,
        })
        .build()
        .unwrap();

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_rate_limit());
    assert_eq!(error.retry_after(), Some(Duration::from_secs(30)));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn never_retries_a_4xx() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .retry(RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            backoff_factor: 2.0,
        })
        .build()
        .unwrap();

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_not_found());
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

// ---------------------------------------------------------------- headers and body

#[tokio::test]
async fn sets_json_content_type_for_a_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let _: Myself = client_for(&server, None)
        .post("/rest/api/3/issue")
        .json(&json!({ "fields": { "summary": "Hello" } }))
        .unwrap()
        .send()
        .await
        .unwrap();

    let request = first_request(&server).await;

    assert_eq!(header_of(&request, "content-type").as_deref(), Some("application/json"));
    assert_eq!(
        String::from_utf8_lossy(&request.body),
        r#"{"fields":{"summary":"Hello"}}"#
    );
}

#[tokio::test]
async fn declares_json_on_a_bodyless_delete_because_jira_answers_415_without_it() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client_for(&server, None)
        .delete("/rest/api/3/issue/X/remotelink")
        .send_empty()
        .await
        .unwrap();

    assert_eq!(
        header_of(&first_request(&server).await, "content-type").as_deref(),
        Some("application/json")
    );
}

#[tokio::test]
async fn declares_nothing_on_a_bodyless_get() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;

    let _: Myself = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send()
        .await
        .unwrap();

    assert_eq!(header_of(&first_request(&server).await, "content-type"), None);
}

#[tokio::test]
async fn lets_a_per_request_header_win_over_a_client_wide_one() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = Client::builder()
        .host(server.uri())
        .header("x-trace", "client")
        .build()
        .unwrap();

    let _: Myself = client
        .get("/rest/api/3/myself")
        .header("x-trace", "request")
        .send()
        .await
        .unwrap();

    assert_eq!(
        header_of(&first_request(&server).await, "x-trace").as_deref(),
        Some("request")
    );
}

#[tokio::test]
async fn sends_a_client_wide_header_when_the_request_names_none() {
    let server = server_answering(json!({ "displayName": "Ada" })).await;
    let client = Client::builder()
        .host(server.uri())
        .header("x-trace", "client")
        .build()
        .unwrap();

    let _: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(
        header_of(&first_request(&server).await, "x-trace").as_deref(),
        Some("client")
    );
}

#[tokio::test]
async fn sends_a_text_body_untouched_under_a_declared_content_type() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client_for(&server, None)
        .put("/rest/api/3/issue/X/comment")
        .content_type("text/plain")
        .body(Body::Text("just text".to_owned()))
        .send_empty()
        .await
        .unwrap();

    let request = first_request(&server).await;

    assert_eq!(header_of(&request, "content-type").as_deref(), Some("text/plain"));
    assert_eq!(String::from_utf8_lossy(&request.body), "just text");
}

#[tokio::test]
async fn sends_a_form_body_as_form_urlencoded() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    client_for(&server, None)
        .put("/rest/api/3/filter/1/columns")
        .body(Body::Form(vec![
            ("columns".to_owned(), "summary".to_owned()),
            ("columns".to_owned(), "status".to_owned()),
        ]))
        .send_empty()
        .await
        .unwrap();

    let request = first_request(&server).await;

    assert_eq!(
        header_of(&request, "content-type").as_deref(),
        Some("application/x-www-form-urlencoded"),
    );
    assert_eq!(String::from_utf8_lossy(&request.body), "columns=summary&columns=status");
}

#[tokio::test]
async fn sends_an_attachment_as_multipart_with_the_field_the_endpoint_reads() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .mount(&server)
        .await;

    let _: serde_json::Value = client_for(&server, None)
        .post("/rest/api/3/issue/X/attachments")
        .header("X-Atlassian-Token", "no-check")
        .body(Body::Multipart(MultipartBody::file(Attachment::new(
            "screenshot.png",
            vec![1u8, 2, 3],
        ))))
        .send()
        .await
        .unwrap();

    let request = first_request(&server).await;
    let content_type = header_of(&request, "content-type").unwrap_or_default();
    let body = String::from_utf8_lossy(&request.body);

    assert!(
        content_type.starts_with("multipart/form-data; boundary="),
        "{content_type}"
    );
    assert!(body.contains(r#"name="file""#), "{body}");
    assert!(body.contains(r#"filename="screenshot.png""#), "{body}");
    assert!(body.contains("image/png"), "{body}");
}

#[tokio::test]
async fn puts_the_query_parameters_on_the_wire() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(query_param("jql", "project = X"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let _: Myself = client_for(&server, None)
        .get("/rest/api/3/search")
        .query("jql", "project = X")
        .query("maxResults", 50)
        .query("startAt", None::<i32>)
        .send()
        .await
        .unwrap();

    let url = first_request(&server).await.url.to_string();

    assert!(url.contains("maxResults=50"), "{url}");
    assert!(!url.contains("startAt"), "{url}");
}

// ------------------------------------- credentials refused behind a status that says otherwise

async fn seraph_server(status: u16, reason: &str, body: serde_json::Value) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(status)
                .set_body_json(body)
                .insert_header("x-seraph-loginreason", reason),
        )
        .mount(&server)
        .await;

    server
}

#[tokio::test]
async fn errors_rather_than_handing_back_the_anonymous_body() {
    let server = seraph_server(200, "AUTHENTICATED_FAILED", json!({ "displayName": "Anonymous" })).await;
    let client = client_for(&server, Some(Auth::api_token("you@example.com", "expired")));

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());
    assert!(error.to_string().contains("anonymous user"), "{error}");
}

#[tokio::test]
async fn reports_the_status_that_was_actually_on_the_wire_not_401() {
    let server = seraph_server(200, "AUTHENTICATED_FAILED", json!({})).await;
    let client = client_for(&server, Some(Auth::api_token("you@example.com", "expired")));

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert_eq!(error.status(), Some(200));
    assert!(error.is_auth());
}

#[tokio::test]
async fn fires_on_a_4xx_too_where_the_status_alone_would_blame_the_request() {
    let server = seraph_server(400, "AUTHENTICATED_FAILED", json!({ "errorMessages": [] })).await;
    let client = client_for(&server, Some(Auth::api_token("you@example.com", "expired")));

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());
    assert_eq!(error.status(), Some(400));
}

#[tokio::test]
async fn counts_a_login_the_instance_refused_outright() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(403)
                .set_body_json(json!({}))
                .insert_header("x-seraph-loginreason", "AUTHENTICATION_DENIED")
                .insert_header(
                    "x-authentication-denied-reason",
                    "CAPTCHA_CHALLENGE; login-url=https://jira/login",
                ),
        )
        .mount(&server)
        .await;

    let client = client_for(&server, Some(Auth::password("ada", "hunter2")));
    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_auth());
    assert!(error.to_string().contains("CAPTCHA_CHALLENGE"), "{error}");
    assert!(error.to_string().contains("may well be correct"), "{error}");
}

#[tokio::test]
async fn leaves_a_client_with_no_credentials_alone() {
    let server = seraph_server(200, "AUTHENTICATED_FAILED", json!({ "displayName": "Anonymous" })).await;

    let myself: Myself = client_for(&server, None)
        .get("/rest/api/3/myself")
        .send()
        .await
        .unwrap();

    assert_eq!(myself.display_name, "Anonymous");
}

#[tokio::test]
async fn does_not_reclassify_a_permission_denial_that_carries_the_header() {
    let server = seraph_server(
        403,
        "AUTHORISATION_FAILED",
        json!({ "errorMessages": ["no permission"] }),
    )
    .await;
    let client = client_for(&server, Some(Auth::api_token("you@example.com", "TOKEN")));

    let error = client.get("/rest/api/3/myself").send::<Myself>().await.unwrap_err();

    assert!(error.is_forbidden());
    assert!(!error.is_auth());
}

#[tokio::test]
async fn gives_the_refresh_hook_the_same_single_attempt_a_401_would() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(header("authorization", "Bearer stale"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({ "displayName": "Anonymous" }))
                .insert_header("x-seraph-loginreason", "AUTHENTICATED_FAILED"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(header("authorization", "Bearer fresh"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "displayName": "Ada" })))
        .mount(&server)
        .await;

    let client = Client::builder()
        .host(server.uri())
        .auth(Auth::bearer("stale"))
        .get_auth_on_401(|| async { Ok(Auth::bearer("fresh")) })
        .build()
        .unwrap();

    let myself: Myself = client.get("/rest/api/3/myself").send().await.unwrap();

    assert_eq!(myself.display_name, "Ada");
}

// ---------------------------------------------------------------- configuration

#[test]
fn host_is_required_unless_the_credentials_route_through_the_gateway() {
    let error = Client::builder().auth(Auth::bearer("PAT")).build().unwrap_err();

    assert!(error.is_config());
    assert!(error.to_string().contains("`host` is required"), "{error}");
}

#[test]
fn host_must_be_a_url() {
    let error = Client::builder().host("your-domain.atlassian.net").build().unwrap_err();

    assert!(error.is_config());
}

#[test]
fn a_trailing_slash_on_the_host_does_not_double_up_in_the_path() {
    let client = Client::builder().host("https://acme.atlassian.net/").build().unwrap();

    assert_eq!(client.host(), Some("https://acme.atlassian.net"));
}

#[test]
fn rejects_a_basic_credential_with_nothing_in_it() {
    let error = Client::builder()
        .host("https://acme.atlassian.net")
        .auth(Auth::api_token("you@example.com", ""))
        .build()
        .unwrap_err();

    assert!(error.is_config());
}
