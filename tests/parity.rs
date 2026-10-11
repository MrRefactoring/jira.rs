#![cfg(all(feature = "cloud", feature = "agile", feature = "server"))]

use jira::core::Client;

#[tokio::test]
async fn delete_subtasks_is_a_boolean_sent_as_one_query_value_on_both_platforms() {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;
    for version in [2, 3] {
        Mock::given(method("DELETE"))
            .and(path(format!("/rest/api/{version}/issue/TEST-1")))
            .and(query_param("deleteSubtasks", "true"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;
    }

    let client = Client::builder().host(server.uri()).build().unwrap();
    jira::cloud::CloudClient::new(client.clone())
        .issues()
        .delete_issue("TEST-1")
        .delete_subtasks(true)
        .send()
        .await
        .unwrap();
    jira::server::ServerClient::new(client).issues().delete_issue("TEST-1").delete_subtasks(true).send().await.unwrap();

    for request in server.received_requests().await.unwrap() {
        assert_eq!(request.url.query_pairs().collect::<Vec<_>>(), [("deleteSubtasks".into(), "true".into())]);
    }
}

macro_rules! parity {
    ($name:ident, $left:path, $right:path, $left_module:path, $right_module:path, $member:ident => ($left_value:expr, $right_value:expr), |$jira:ident, $surface:ident| $body:block) => {
        mod $name {
            use super::*;

            fn left() {
                use $left_module as $surface;
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$left>::new(client);
                let $member = $left_value;
                $body
            }

            fn right() {
                use $right_module as $surface;
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$right>::new(client);
                let $member = $right_value;
                $body
            }

            #[test]
            fn both_compile() {
                let _ = (left as fn(), right as fn());
            }
        }
    };
    ($name:ident, $left:path, $right:path, $left_module:path, $right_module:path, |$jira:ident, $surface:ident| $body:block) => {
        mod $name {
            use super::*;

            fn left() {
                use $left_module as $surface;
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$left>::new(client);
                $body
            }

            fn right() {
                use $right_module as $surface;
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$right>::new(client);
                $body
            }

            #[test]
            fn both_compile() {
                let _ = (left as fn(), right as fn());
            }
        }
    };
    ($name:ident, $left:path, $right:path, |$jira:ident| $body:block) => {
        mod $name {
            use super::*;

            fn left() {
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$left>::new(client);
                $body
            }

            fn right() {
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$right>::new(client);
                $body
            }

            #[test]
            fn both_compile() {
                left();
                right();
            }
        }
    };
}

parity!(
    typed_projects,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        use futures_util::StreamExt;
        drop(async {
            let item: surface::Project = jira.projects().search_projects().stream().next().await.unwrap().unwrap();
            let _ = item;
        });
    }
);

parity!(
    typed_issues,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        use futures_util::StreamExt;
        drop(async {
            let item: surface::Issue = jira
                .issue_search()
                .search_issues()
                .jql("project = X")
                .fields(["summary"])
                .expand(["changelog"])
                .stream()
                .next()
                .await
                .unwrap()
                .unwrap();
            let _ = (item.key, item.changelog);
        });
    }
);

parity!(
    typed_comment,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        drop(async {
            let _: surface::Comment = jira.issue_comments().get_comment("X-1", "1").send().await.unwrap();
        });
    }
);

parity!(
    typed_worklog,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        drop(async {
            let _: surface::Worklog = jira.issue_worklogs().get_worklog("X-1", "1").send().await.unwrap();
        });
    }
);

parity!(
    typed_version,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        drop(async {
            let _: surface::Version = jira.project_versions().get_version("1").send().await.unwrap();
        });
    }
);

parity!(
    typed_sprint,
    jira::agile::AgileClient,
    jira::server::ServerClient,
    jira::agile,
    jira::server,
    |jira, surface| {
        drop(async {
            let _: surface::Sprint = jira.sprint().get_sprint(1).send().await.unwrap();
        });
    }
);

parity!(
    typed_board,
    jira::agile::AgileClient,
    jira::server::ServerClient,
    jira::agile,
    jira::server,
    |jira, surface| {
        use futures_util::StreamExt;
        drop(async {
            let _: surface::Board = jira.board().get_all_boards().stream().next().await.unwrap().unwrap();
        });
    }
);

parity!(agile_create_sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().create_sprint("Sprint 1").origin_board_id(7));
});

parity!(agile_update_sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().update_sprint(1, "Sprint 1", "active"));
});

parity!(agile_partial_sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().partially_update_sprint(1).name("x"));
});

parity!(agile_swap_sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().swap_sprint(1).sprint_to_swap_with(2));
});

parity!(agile_move_to_backlog, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.backlog().move_issues_to_backlog(["X-1"]));
});

parity!(agile_move_to_epic, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.epic().move_issues_to_epic("E-1").issues(["X-1"]));
});

parity!(agile_remove_from_epic, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.epic().remove_issues_from_epic(["X-1"]));
});

parity!(agile_estimate_issue, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.issue().estimate_issue_for_board("X-1").value("3"));
});

parity!(agile_move_to_sprint_and_rank, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().move_issues_to_sprint_and_rank(1, Default::default()));
});

parity!(
    typed_user,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    |jira, surface| {
        use futures_util::StreamExt;
        drop(async {
            let _: surface::User = jira.user_search().find_users().stream().next().await.unwrap().unwrap();
        });
    }
);

macro_rules! parity_with_value {
    ($name:ident, $left:path, $right:path, $left_value:expr, $right_value:expr, |$jira:ident, $value:ident| $body:block) => {
        mod $name {
            use super::*;

            fn left() {
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$left>::new(client);
                let $value = $left_value;
                $body
            }

            fn right() {
                let client = Client::builder().host("https://example.com").build().unwrap();
                let $jira = <$right>::new(client);
                let $value = $right_value;
                $body
            }

            #[test]
            fn both_compile() {
                let _ = (left as fn(), right as fn());
            }
        }
    };
}

parity!(projects, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    let mut projects = jira.projects().search_projects().stream();
    let _ = projects.as_mut();
});

parity!(issue_search, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_search().search_issues().stream());
    drop(jira.issue_search().search_issues().jql("project = X").fields(["summary"]));
});

parity!(issue_search_expand, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_search().search_issues().jql("project = X").fields(["summary"]).expand(["changelog"]).stream());
});

parity_with_value!(
    issue_search_post_stream,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud::models::SearchAndReconcileRequest::default(),
    jira::server::models::SearchRequest::default(),
    |jira, body| {
        drop(jira.issue_search().search_issues_post(body).stream());
    }
);

parity!(list_expansions, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issues().get_issue("X-1").expand(["changelog", "names"]));
    drop(jira.issue_comments().get_comments("X-1").expand(["renderedBody"]));
    drop(jira.projects().get_project("X").expand(["description"]));
    drop(jira.project_versions().get_project_versions_paginated("X").expand(["operations"]).stream());
});

parity!(issues, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issues().get_issue("X-1").update_history(true).fields(["summary"]).properties(["key"]));
});

parity_with_value!(
    assign_issue,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    Some("account-id".to_owned()),
    serde_json::from_value::<jira::server::models::User>(serde_json::json!({})).unwrap(),
    |jira, body| {
        drop(jira.issues().assign_issue("X-1", body));
    }
);

parity!(board, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.board().get_all_boards().stream());
    drop(jira.board().get_all_sprints(1).stream());
    drop(jira.board().get_issues_for_board(1).fields(["summary"]).stream());
    drop(jira.board().get_board_property("7", "key"));
});

#[test]
fn agile_full_projects_has_a_single_page_stream() {
    let client = Client::builder().host("https://example.com").build().unwrap();
    let jira = jira::agile::AgileClient::new(client);
    assert!(jira.board().get_projects_full(1).config().unwrap().query.is_empty());
    drop(jira.board().get_projects_full(1).stream());
}

parity!(sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().get_issues_for_sprint(1).fields(["summary", "status"]).stream());
    drop(jira.sprint().get_property("7", "key"));
});

parity!(dashboards, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.dashboards().get_all_dashboards().start_at(0).max_results(50).stream());
});

parity!(issue_worklogs, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_worklogs().get_issue_worklog("X-1").stream());
});

parity!(
    add_worklog,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    comment => (Some(surface::WorklogInputComment::Variant1("work".to_owned())), Some("work".to_owned())),
    |jira, surface| {
        drop(jira.issue_worklogs().add_worklog("X-1", surface::WorklogInput {
            comment,
            time_spent: Some("1h".to_owned()),
            ..Default::default()
        }));
    }
);

parity!(project_versions, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.project_versions().get_project_versions("X"));
    drop(jira.project_versions().get_project_versions_paginated("X").stream());
});

parity_with_value!(
    create_version,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    serde_json::from_value::<jira::cloud::models::Version>(serde_json::json!({})).unwrap(),
    serde_json::from_value::<jira::server::models::Version>(serde_json::json!({})).unwrap(),
    |jira, body| {
        drop(jira.project_versions().create_version(body));
    }
);

parity_with_value!(
    create_filter,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    serde_json::from_value::<jira::cloud::models::Filter>(serde_json::json!({})).unwrap(),
    serde_json::from_value::<jira::server::models::Filter>(serde_json::json!({})).unwrap(),
    |jira, body| {
        drop(jira.filters().create_filter(body));
    }
);

parity!(issue_comments, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_comments().get_comments("X-1").stream());
});

parity!(
    add_comment,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud,
    jira::server,
    body => (Some(surface::CommentInputBody::Variant1("hello".to_owned())), Some("hello".to_owned())),
    |jira, surface| {
        drop(jira.issue_comments().add_comment("X-1", surface::CommentInput {
            body,
            ..Default::default()
        }));
    }
);

parity!(user_search, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.user_search().find_users().stream());
    drop(jira.user_search().find_assignable_users());
});

#[test]
fn server_issue_lists_use_one_comma_separated_query_value() {
    let client = Client::builder().host("https://example.com").build().unwrap();
    let jira = jira::server::ServerClient::new(client);
    let issue = jira.issues().get_issue("X-1").fields(["summary", "status"]).properties(["a", "b"]).config().unwrap();
    assert!(issue.query.contains(&("fields".to_owned(), jira::core::QueryValue::Scalar("summary,status".to_owned()))));
    assert!(issue.query.contains(&("properties".to_owned(), jira::core::QueryValue::Scalar("a,b".to_owned()))));
    let agile = jira.board().get_issues_for_board(7).fields(["summary", "status"]).config().unwrap();
    assert!(agile.query.contains(&("fields".to_owned(), jira::core::QueryValue::Scalar("summary,status".to_owned()))));
}
