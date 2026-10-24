use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use jira::server::{AddGroup, Avatar, CreatedIssue, Filter, IssueUpdateDetails, ProjectInput, ServerClient, UserWrite};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use tokio::sync::OnceCell;

use crate::harness::{ResourceTracker, project_key, require_server_env, run_id, server, test_name};

const SCRUM_TEMPLATE: &str = "com.pyxis.greenhopper.jira:gh-scrum-template";

const BUSINESS_TEMPLATE: &str = "com.atlassian.jira-core-project-templates:jira-core-project-management";

pub async fn software_licensed() -> bool {
    static LICENSED: OnceCell<bool> = OnceCell::const_new();

    *LICENSED
        .get_or_init(|| async {
            let licensed = server().application_roles().get_application_role("jira-software").send().await.is_ok();

            if !licensed {
                eprintln!(
                    "[live] Jira Software is not licensed on this instance, so every suite needing a board, a \
sprint or an issue in a software project stands down. Atlassian's published Jira Software Data Center timebomb is \
expired and they have not replaced it, so a licence of your own in docker/jira-dc/timebomb-license.txt is what \
brings these back."
                );
            }

            licensed
        })
        .await
}

pub fn touch<T>(outcome: jira::Result<T>) -> Option<T> {
    match outcome {
        Ok(value) => Some(value),
        Err(error) => {
            assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "a refusal is typed: {error}");

            None
        }
    }
}

pub fn admin_username() -> String {
    require_server_env().username
}

fn unique_project_key() -> String {
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    let base: String = project_key("").chars().take(7).collect();

    format!("{base}{ordinal:02X}")
}

pub struct TestProject {
    pub id: i64,
    pub key: String,
}

pub async fn scrum_project(tracker: &mut ResourceTracker, label: &str) -> TestProject {
    create_project(tracker, "software", SCRUM_TEMPLATE, label).await
}

pub async fn business_project(tracker: &mut ResourceTracker, label: &str) -> TestProject {
    create_project(tracker, "business", BUSINESS_TEMPLATE, label).await
}

async fn create_project(tracker: &mut ResourceTracker, type_key: &str, template_key: &str, label: &str) -> TestProject {
    let key = unique_project_key();
    let name: String = test_name(label).chars().take(80).collect();

    let created = server()
        .projects()
        .create_project(ProjectInput {
            key: Some(key.clone()),
            name: Some(name),
            lead: Some(admin_username()),
            project_type_key: Some(type_key.to_owned()),
            project_template_key: Some(template_key.to_owned()),
            ..ProjectInput::default()
        })
        .send()
        .await
        .expect("the instance accepts a project of the suite's own");

    let doomed = key.clone();

    tracker.defer(move || {
        let key = doomed.clone();

        async move { server().projects().delete_project(key).send().await }
    });

    TestProject { id: created.id.expect("a created project carries an id"), key }
}

pub async fn create_issue(tracker: &mut ResourceTracker, fields: serde_json::Value) -> CreatedIssue {
    let fields = fields
        .as_object()
        .expect("issue fields are an object")
        .iter()
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();

    let created = server()
        .issues()
        .create_issue(IssueUpdateDetails { fields: Some(fields), ..IssueUpdateDetails::default() })
        .send()
        .await
        .expect("the project accepts a new issue");

    let key = created.key.clone().expect("a created issue carries a key");

    tracker.defer(move || {
        let key = key.clone();

        async move { server().issues().delete_issue(key).delete_subtasks(true).send().await }
    });

    created
}

pub async fn create_task(tracker: &mut ResourceTracker, project_key: &str, label: &str) -> CreatedIssue {
    create_issue(
        tracker,
        json!({
            "project": { "key": project_key },
            "issuetype": { "name": "Task" },
            "summary": test_name(label),
        }),
    )
    .await
}

pub async fn create_epic(tracker: &mut ResourceTracker, project_key: &str, label: &str) -> CreatedIssue {
    let fields = server().issue_fields().get_fields().send().await.expect("the instance lists the fields it has");
    let epic_name = fields
        .iter()
        .find(|field| field.name.as_deref() == Some("Epic Name"))
        .and_then(|field| field.id.clone())
        .expect("a Scrum project carries an \"Epic Name\" field");

    let summary = test_name(label);
    let mut fields = serde_json::Map::new();

    fields.insert("project".to_owned(), json!({ "key": project_key }));
    fields.insert("issuetype".to_owned(), json!({ "name": "Epic" }));
    fields.insert("summary".to_owned(), json!(summary));
    fields.insert(epic_name, json!(summary));

    create_issue(tracker, serde_json::Value::Object(fields)).await
}

pub async fn board_of(project_key: &str) -> i64 {
    for _ in 0..30 {
        let boards = server()
            .board()
            .get_all_boards()
            .project_key_or_id(project_key)
            .max_results(1)
            .send()
            .await
            .expect("the board listing is accepted");

        if let Some(id) = boards.values.first().and_then(|board| board.id) {
            return id;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    panic!("[live] the Scrum template never produced a board for {project_key}");
}

pub async fn create_test_filter(tracker: &mut ResourceTracker, label: &str, jql: &str) -> Filter {
    let filter = server()
        .filters()
        .create_filter(Filter { name: Some(test_name(label)), jql: Some(jql.to_owned()), ..Filter::default() })
        .send()
        .await
        .expect("the account may create a filter of its own");

    let id = filter.id.clone().expect("a created filter carries an id");
    let doomed = id.clone();

    tracker.defer(move || {
        let id = doomed.clone();

        async move { server().filters().delete_filter(id.parse().expect("a filter id is a number")).send().await }
    });

    filter
}

pub async fn create_test_user(tracker: &mut ResourceTracker) -> String {
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    let name: String = format!("jrs-{}-{ordinal}", run_id()).chars().take(30).collect();

    server()
        .users()
        .create_user(UserWrite {
            name: Some(name.clone()),
            password: Some("Correct-Horse-Battery-1".to_owned()),
            email_address: Some(format!("{name}@example.com")),
            display_name: Some("created by the users suite".to_owned()),
            ..UserWrite::default()
        })
        .send()
        .await
        .expect("a self-hosted Jira owns its directory and may be told to create a user");

    let doomed = name.clone();

    tracker.defer(move || {
        let name = doomed.clone();

        async move { server().users().remove_user().username(name).send().await }
    });

    name
}

pub async fn create_test_group(tracker: &mut ResourceTracker) -> String {
    static NEXT: AtomicUsize = AtomicUsize::new(0);

    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    let name: String = format!("{} {ordinal}", test_name("grp")).chars().take(60).collect();

    server()
        .groups()
        .create_group(AddGroup { name: Some(name.clone()) })
        .send()
        .await
        .expect("the administrator may create a group");

    let doomed = name.clone();

    tracker.defer(move || {
        let name = doomed.clone();

        async move { server().groups().remove_group(name).send().await }
    });

    name
}

pub fn property_body() -> HashMap<String, serde_json::Value> {
    [("written".to_owned(), json!(true))].into_iter().collect()
}

pub fn property_value() -> serde_json::Value {
    json!({ "written": true })
}

pub fn avatar_image() -> jira::Attachment {
    jira::Attachment::new("avatar.png", include_bytes!("avatar.png").as_slice())
}

pub async fn avatar_session() -> ServerClient {
    let env = require_server_env();
    let mut headers = HeaderMap::new();
    headers.insert("X-Atlassian-Token", HeaderValue::from_static("no-check"));
    let http = reqwest::Client::builder()
        .cookie_store(true)
        .default_headers(headers)
        .timeout(Duration::from_secs(60))
        .build()
        .expect("an avatar client stores session cookies");

    http.post(format!("{}/rest/tsv/latest/authenticate", env.host))
        .json(&json!({ "username": env.username, "password": env.password }))
        .send()
        .await
        .expect("the session endpoint answers")
        .error_for_status()
        .expect("the avatar tests authenticate with a persistent session");

    let client = jira::Client::builder()
        .host(env.host)
        .http_client(http)
        .build()
        .expect("the session describes a usable client");
    ServerClient::new(client)
}

pub async fn verify_avatar(
    client: &ServerClient,
    kind: &str,
    owner: &str,
    avatar: Avatar,
    tracker: &mut ResourceTracker,
) {
    let id = avatar.id.expect("a created avatar carries an id");
    let numeric_id = id.parse::<i64>().expect("an avatar id is numeric");
    let cleanup_client = client.client().clone();
    let cleanup_kind = kind.to_owned();
    let cleanup_owner = owner.to_owned();
    tracker.defer(move || {
        let client = ServerClient::new(cleanup_client.clone());
        let kind = cleanup_kind.clone();
        let owner = cleanup_owner.clone();
        async move { client.avatars().delete_avatar(kind, owner, numeric_id).send().await }
    });

    let avatars = client.avatars().get_avatars(kind, owner).send().await.expect("the owner's avatars read back");
    assert!(
        avatars.custom.unwrap_or_default().iter().any(|avatar| avatar.id.as_ref() == Some(&id)),
        "the created avatar appears in the owner's custom avatars",
    );

    client.avatars().delete_avatar(kind, owner, numeric_id).send().await.expect("the created avatar can be deleted");
    let avatars = client.avatars().get_avatars(kind, owner).send().await.expect("avatars read back after deletion");
    assert!(
        avatars.custom.unwrap_or_default().iter().all(|avatar| avatar.id.as_ref() != Some(&id)),
        "the deleted avatar is absent",
    );
}
