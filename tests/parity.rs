#![cfg(all(feature = "cloud", feature = "agile", feature = "server"))]

use jira::core::Client;

macro_rules! parity {
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

parity!(projects, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    let mut projects = jira.projects().search_projects().stream();
    let _ = projects.as_mut();
});

parity!(issue_search, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_search().search_issues().stream());
});

parity!(board, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.board().get_all_boards().stream());
    drop(jira.board().get_all_sprints(1).stream());
});

parity!(sprint, jira::agile::AgileClient, jira::server::ServerClient, |jira| {
    drop(jira.sprint().get_issues_for_sprint(1).stream());
});

parity!(issue_worklogs, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_worklogs().get_issue_worklog("X-1").stream());
});

parity!(project_versions, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.project_versions().get_project_versions_paginated("X").stream());
});

parity!(issue_comments, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.issue_comments().get_comments("X-1").stream());
});

parity!(user_search, jira::cloud::CloudClient, jira::server::ServerClient, |jira| {
    drop(jira.user_search().find_users().stream());
});
