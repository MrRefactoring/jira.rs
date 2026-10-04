#![cfg(all(feature = "server", feature = "agile", feature = "cloud", feature = "assets-server"))]

use futures_util::TryStreamExt;
use jira::core::Client;
use serde_json::{Value, json};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn client_for(server: &MockServer) -> Client {
    Client::builder().host(server.uri()).build().unwrap()
}

fn query(request: &Request, name: &str) -> Option<String> {
    request.url.query_pairs().find(|(key, _)| key == name).map(|(_, value)| value.into_owned())
}

macro_rules! suite {
    ($module:ident, $client:expr, $api:ident, $request:expr, $path:literal, $key:literal, [$($expected:expr),+], $body:expr, $first_count:expr, $identity:literal, [$($id:expr),+]) => {
        mod $module {
            use super::*;

            fn body(index: usize, empty: bool) -> Value {
                ($body)(index, empty)
            }

            async fn mount(server: &MockServer, fail_second: bool, empty: bool) {
                Mock::given(method("GET"))
                    .and(path($path))
                    .respond_with(move |request: &Request| {
                        let position = query(request, $key);
                        let positions = [$($expected),+];
                        let index = positions.iter().position(|candidate| *candidate == position.as_deref()).unwrap_or(99);
                        if index == 99 || (fail_second && index == 1) {
                            ResponseTemplate::new(500)
                        } else {
                            ResponseTemplate::new(200).set_body_json(body(index, empty))
                        }
                    })
                    .mount(server)
                    .await;
            }

            async fn positions(server: &MockServer) -> Vec<Option<String>> {
                server.received_requests().await.unwrap().iter().map(|request| query(request, $key)).collect()
            }

            #[tokio::test]
            async fn walks_pages_in_order() {
                let server = MockServer::start().await;
                mount(&server, false, false).await;
                let client = client_for(&server);
                let $api = $client(client);
                let items: Vec<_> = ($request).stream().try_collect().await.unwrap();
                assert_eq!(items.len(), $first_count + 1);
                let actual: Vec<Value> = items.iter().map(|item| serde_json::to_value(item).unwrap()[$identity].clone()).collect();
                assert_eq!(actual, vec![$(json!($id)),+]);
                let expected = [$($expected.map(str::to_owned)),+];
                assert_eq!(positions(&server).await, expected);
            }

            #[tokio::test]
            async fn empty_result_sends_one_request() {
                let server = MockServer::start().await;
                mount(&server, false, true).await;
                let client = client_for(&server);
                let $api = $client(client);
                let items: Vec<_> = ($request).stream().try_collect().await.unwrap();
                assert!(items.is_empty());
                assert_eq!(positions(&server).await.len(), 1);
            }

            #[tokio::test]
            async fn second_page_failure_ends_stream() {
                let server = MockServer::start().await;
                mount(&server, true, false).await;
                let client = client_for(&server);
                let $api = $client(client);
                let mut stream = ($request).stream();
                for _ in 0..$first_count {
                    assert!(stream.try_next().await.unwrap().is_some());
                }
                assert!(stream.try_next().await.is_err());
                assert!(stream.try_next().await.unwrap().is_none());
                assert_eq!(positions(&server).await.len(), 2);
            }

            #[tokio::test]
            async fn dropping_after_first_item_sends_no_more() {
                let server = MockServer::start().await;
                mount(&server, false, false).await;
                let client = client_for(&server);
                let $api = $client(client);
                let mut stream = ($request).stream();
                assert!(stream.try_next().await.unwrap().is_some());
                drop(stream);
                assert_eq!(positions(&server).await.len(), 1);
            }
        }
    };
}

suite!(
    server_search,
    jira::server::ServerClient::new,
    api,
    api.issue_search().search(),
    "/rest/api/2/search",
    "startAt",
    [Some("0"), Some("1")],
    |index: usize, empty: bool| json!({"startAt": index, "total": if empty {0} else {2}, "issues": if empty {vec![]} else {vec![json!({"key": format!("X-{}", index)})]}}),
    1,
    "key",
    ["X-0", "X-1"]
);

suite!(
    board_sprints,
    jira::server::ServerClient::new,
    api,
    api.board().get_all_sprints(1),
    "/rest/agile/1.0/board/1/sprint",
    "startAt",
    [Some("0"), Some("1")],
    |index: usize, empty: bool| json!({"startAt": index, "maxResults": 1, "isLast": empty || index == 1, "values": if empty {vec![]} else {vec![json!({"id": index + 1, "state": "active"})]}}),
    1,
    "id",
    [1, 2]
);

#[tokio::test]
async fn cloud_changed_worklogs_stop_at_last_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/api/3/worklog/updated"))
        .respond_with(|request: &Request| match query(request, "since").as_deref() {
            Some("10") => ResponseTemplate::new(200)
                .set_body_json(json!({"values": [{"worklogId": 1}], "until": 20, "lastPage": false})),
            Some("20") => ResponseTemplate::new(200)
                .set_body_json(json!({"values": [{"worklogId": 2}], "until": 30, "lastPage": true})),
            _ => ResponseTemplate::new(500),
        })
        .mount(&server)
        .await;

    let client = client_for(&server);
    let cloud = jira::cloud::CloudClient::new(client);
    let items: Vec<_> =
        cloud.issue_worklogs().get_ids_of_worklogs_modified_since().since(10).stream().try_collect().await.unwrap();

    assert_eq!(items.iter().map(|item| item.worklog_id).collect::<Vec<_>>(), [Some(1), Some(2)]);
    assert_eq!(
        server.received_requests().await.unwrap().iter().map(|request| query(request, "since")).collect::<Vec<_>>(),
        [Some("10".to_owned()), Some("20".to_owned())]
    );
}

suite!(
    cloud_users,
    jira::cloud::CloudClient::new,
    api,
    api.user_search().find_users().max_results(2),
    "/rest/api/3/user/search",
    "startAt",
    [Some("0"), Some("2")],
    |index: usize, empty: bool| if empty {
        json!([])
    } else if index == 0 {
        json!([{"accountId": "a"}, {"accountId": "b"}])
    } else {
        json!([{"accountId": "c"}])
    },
    2,
    "accountId",
    ["a", "b", "c"]
);

#[tokio::test]
async fn server_find_users_walks_two_pages_then_an_empty_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/api/2/user/search"))
        .respond_with(|request: &Request| match query(request, "startAt").as_deref() {
            Some("0") => ResponseTemplate::new(200).set_body_json(json!([{"name": "a"}, {"name": "b"}])),
            Some("2") => ResponseTemplate::new(200).set_body_json(json!([{"name": "c"}, {"name": "d"}])),
            Some("4") => ResponseTemplate::new(200).set_body_json(json!([])),
            _ => ResponseTemplate::new(500),
        })
        .mount(&server)
        .await;

    let api = jira::server::ServerClient::new(client_for(&server));
    let users: Vec<_> = api.users().find_users().max_results(2).stream().try_collect().await.unwrap();

    assert_eq!(
        users.iter().map(|user| user.name.as_deref()).collect::<Vec<_>>(),
        [Some("a"), Some("b"), Some("c"), Some("d")]
    );
    assert_eq!(
        server.received_requests().await.unwrap().iter().map(|request| query(request, "startAt")).collect::<Vec<_>>(),
        [Some("0".to_owned()), Some("2".to_owned()), Some("4".to_owned())]
    );
}

suite!(
    cloud_search,
    jira::cloud::CloudClient::new,
    api,
    api.issue_search().search_issues(),
    "/rest/api/3/search/jql",
    "nextPageToken",
    [None, Some("next")],
    |index: usize, empty: bool| json!({"issues": if empty {vec![]} else {vec![json!({"key": format!("X-{}", index)})]}, "nextPageToken": if empty || index == 1 {None} else {Some("next")}}),
    1,
    "key",
    ["X-0", "X-1"]
);

#[test]
fn requested_stream_signatures_compile() {
    let client = Client::builder().host("https://example.invalid").build().unwrap();
    let server = jira::server::ServerClient::new(client.clone());
    let cloud = jira::cloud::CloudClient::new(client.clone());
    let assets_server = jira::assets_server::AssetsServerClient::new(client.clone());
    let agile = jira::agile::AgileClient::new(client);

    drop(server.issue_search().search().stream());
    drop(server.sprint().get_issues_for_sprint(1).stream());
    drop(server.issues().get_issue_worklog("X-1").stream());
    drop(server.users().get_user_list().stream());
    drop(server.users().find_users().stream());
    drop(server.dashboards().list().start_at(50).stream());
    drop(server.issues().get_comments("X-1").max_results(10).stream());
    drop(server.screens().get_all_screens().stream());
    drop(server.groups().get_users_from_group("g").stream());
    drop(cloud.issue_worklogs().get_issue_worklog("X-1").stream());
    drop(cloud.issue_comments().get_comments("X-1").stream());
    drop(cloud.user_search().find_users().stream());
    drop(cloud.issue_notification_schemes().get_notification_schemes().start_at(0).stream());
    drop(cloud.issue_priorities().search_priorities().stream());
    drop(assets_server.aql().find_objects().stream());
    drop(assets_server.objects().get_archived_objects().stream());
    drop(agile.board().get_all_sprints(1).stream());
    drop(agile.sprint().get_issues_for_sprint(1).stream());
}
