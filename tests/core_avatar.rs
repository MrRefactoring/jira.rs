#![cfg(feature = "server")]

use jira::core::{Attachment, Client, Error, Result};
use jira::server::ServerClient;
use serde_json::{Value, json};
use wiremock::matchers::{header, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

const OPERATIONS: [&str; 4] = ["universal", "project", "issue type", "user"];

async fn fixture(response: ResponseTemplate, count: u64) -> (MockServer, ServerClient) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(header("accept", "text/html"))
        .and(header("x-atlassian-token", "no-check"))
        .respond_with(response)
        .expect(count)
        .mount(&server)
        .await;
    let client = Client::builder().host(server.uri()).build().unwrap();
    (server, ServerClient::new(client))
}

async fn upload(client: &ServerClient, operation: &str) -> Result<Value> {
    let files = [Attachment::new("avatar.png", vec![1u8, 2, 3])];
    match operation {
        "universal" => Ok(serde_json::to_value(
            client.avatars().store_temporary_avatar_using_multi_part("project", "10000", files).send().await?,
        )?),
        "project" => client.projects().store_temporary_project_avatar_using_multi_part("10000", files).send().await,
        "issue type" => {
            client.issue_types().store_temporary_issue_type_avatar_using_multi_part("10000", files).send().await
        }
        "user" => client.users().store_temporary_user_avatar_using_multi_part(files).username("admin").send().await,
        _ => unreachable!(),
    }
}

fn html(value: Value, status: u16) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_raw(
        format!("<html><body><textarea id=\"upload\">{value}</textarea></body></html>"),
        "text/html;charset=UTF-8",
    )
}

#[tokio::test]
async fn four_avatar_uploads_decode_html_and_send_multipart_with_the_required_headers() {
    let response = html(
        json!({
            "url": "/secure/temporaryavatar?cropped=true&magic=123",
            "cropperWidth": "64", "cropperOffsetX": "9", "cropperOffsetY": "-9", "isCroppingNeeded": "true",
        }),
        201,
    );
    let (server, client) = fixture(response, 4).await;
    let expected = json!({
        "url": "/secure/temporaryavatar?cropped=true&magic=123",
        "cropperWidth": 64, "cropperOffsetX": 9, "cropperOffsetY": -9, "needsCropping": true,
    });
    for operation in OPERATIONS {
        assert_eq!(upload(&client, operation).await.unwrap(), expected, "{operation}");
    }
    for request in server.received_requests().await.unwrap() {
        assert!(request.headers["content-type"].to_str().unwrap().starts_with("multipart/form-data; boundary="));
        assert!(request.body.windows(13).any(|bytes| bytes == b"name=\"avatar\""));
    }
}

#[tokio::test]
async fn four_avatar_uploads_still_accept_json_cropping_instructions() {
    let value = json!({ "cropperWidth": 64, "needsCropping": false });
    let (_server, client) = fixture(ResponseTemplate::new(201).set_body_json(&value), 4).await;
    for operation in OPERATIONS {
        assert_eq!(upload(&client, operation).await.unwrap(), value, "{operation}");
    }
}

#[tokio::test]
async fn unmodelled_avatar_uploads_preserve_an_automatically_created_avatar_id() {
    let value = json!({ "id": "10600" });
    let (_server, client) = fixture(html(value.clone(), 201), 3).await;
    for operation in &OPERATIONS[1..] {
        assert_eq!(upload(&client, operation).await.unwrap(), value, "{operation}");
    }
}

#[tokio::test]
async fn four_avatar_uploads_reject_errors_embedded_in_http_success() {
    for body in [
        json!({ "errorMessages": ["The image cannot be decoded"] }),
        json!({ "errors": { "avatar": "Invalid image" } }),
    ] {
        let (_server, client) = fixture(html(body.clone(), 200), 4).await;
        for operation in OPERATIONS {
            let error = upload(&client, operation).await.unwrap_err();
            let Error::Api { details, .. } = error else { panic!("Expected an API error") };
            assert_eq!(details.status, 200);
            assert_eq!(details.body, body);
        }
    }
}

#[tokio::test]
async fn a_malformed_avatar_response_is_a_schema_error() {
    let (_server, client) =
        fixture(ResponseTemplate::new(201).set_body_raw("<html>unavailable</html>", "text/html"), 1).await;
    let error = upload(&client, "universal").await.unwrap_err();
    assert!(matches!(error, Error::SchemaMismatch { .. }));
}

#[tokio::test]
async fn typed_avatar_instructions_are_validated_after_html_decoding() {
    let (_server, client) = fixture(html(json!({ "cropperWidth": "invalid" }), 201), 1).await;
    let error = upload(&client, "universal").await.unwrap_err();
    assert!(matches!(error, Error::SchemaMismatch { .. }));
    assert_eq!(error.schema_report().unwrap().issues[0].path, "cropperWidth");
}

#[tokio::test]
async fn the_raw_escape_hatch_preserves_the_original_html() {
    let body = "<textarea>{\"isCroppingNeeded\":\"true\"}</textarea>";
    let (_server, client) = fixture(ResponseTemplate::new(201).set_body_raw(body, "text/html"), 1).await;
    let value = client
        .avatars()
        .store_temporary_avatar_using_multi_part("project", "10000", [Attachment::new("avatar.png", vec![1u8])])
        .send_raw()
        .await
        .unwrap();
    assert_eq!(value, Value::String(body.to_owned()));
}

#[tokio::test]
async fn an_http_refusal_is_reported_before_avatar_decoding() {
    let (_server, client) = fixture(ResponseTemplate::new(400).set_body_raw("XSRF check failed", "text/html"), 1).await;
    let error = upload(&client, "universal").await.unwrap_err();
    let Error::Api { details, .. } = error else { panic!("Expected an API error") };
    assert_eq!(details.status, 400);
}
