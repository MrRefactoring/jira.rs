#![cfg(all(feature = "cloud", feature = "agile", feature = "server"))]

use jira::core::Client;

macro_rules! parity {
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
                .expand(surface::SearchIssuesRequestExpand::Other(serde_json::json!("changelog")))
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

parity_with_value!(
    issue_search_expand,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    jira::cloud::SearchIssuesRequestExpand::Other(serde_json::json!("changelog")),
    jira::server::SearchIssuesRequestExpand::Other(serde_json::json!("changelog")),
    |jira, expand| {
        drop(jira.issue_search().search_issues().jql("project = X").expand(expand));
    }
);

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

parity_with_value!(
    add_worklog,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    serde_json::from_value::<jira::cloud::models::WorklogInput>(serde_json::json!({})).unwrap(),
    serde_json::from_value::<jira::server::models::Worklog>(serde_json::json!({})).unwrap(),
    |jira, body| {
        drop(jira.issue_worklogs().add_worklog("X-1", body));
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

parity_with_value!(
    add_comment,
    jira::cloud::CloudClient,
    jira::server::ServerClient,
    serde_json::from_value::<jira::cloud::models::CommentInput>(serde_json::json!({})).unwrap(),
    serde_json::from_value::<jira::server::models::Comment>(serde_json::json!({})).unwrap(),
    |jira, body| {
        drop(jira.issue_comments().add_comment("X-1", body));
    }
);

parity!(user_search, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.user_search().find_users().stream());
    drop(jira.user_search().find_assignable_users().stream());
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
