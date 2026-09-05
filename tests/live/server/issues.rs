//! The write path through the issues module, end to end against a real Data Center instance.
//!
//! What a read can never prove is what this is for: that a body serialises the way Jira expects, that a transition
//! moves an issue, that wiki markup goes out as a plain string where Cloud would need a document, and that what comes
//! back matches the schema the library declares for it.
//!
//! Each test owns everything it touches, project included. Nothing is shared, because these tests delete, archive and
//! transition what they create, and because there is no ambient project on a bare instance to borrow.

use std::collections::HashMap;

use jira::server::{
    CommentJson, IssueLinkTypeJson, IssueRefJson, IssueSubTaskMovePosition, IssueUpdate, IssuesUpdate,
    LinkIssueRequestJson, NotificationJson, RemoteIssueLinkCreateOrUpdateRequest, RemoteObject,
    RemoteReciprocalIssueLinkCreateRequest, SearchRequest, SearchRequest2Fields, ToJson, Transition, User, Worklog,
    WorklogIdsRequest,
};
use serde_json::json;

use super::fixtures::{
    admin_username, create_issue, create_task, property_body, property_value, scrum_project, software_licensed, touch,
};
use crate::harness::{ResourceTracker, poll_until, server, test_name};

/// The global id the remote issue links in this suite are written under.
const REMOTE_LINK_GLOBAL_ID: &str = "jrs-remote-issue-link";

/// The fields of an issue, in the map shape `IssueUpdate` takes.
fn fields(value: serde_json::Value) -> HashMap<String, serde_json::Value> {
    value
        .as_object()
        .expect("issue fields are an object")
        .iter()
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect()
}

/// Creation, the wiki markup that goes with it, an edit, an assignment, and the metadata behind both.
///
/// Four of the TypeScript suite's tests in one, because they are one sequence over one issue and Rust has no
/// `beforeAll` to build that issue once for all of them.
#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn creates_an_issue_in_wiki_markup_and_edits_it() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "issues suite").await;
    let username = admin_username();

    let issue = create_issue(
        &mut tracker,
        json!({
            "project": { "key": project.key },
            "issuetype": { "name": "Task" },
            "summary": test_name("the issues suite"),
            // Data Center takes wiki markup as a plain string. The Cloud surface would reject this and want a
            // document, which is why there is no `Document` type on this surface at all.
            "description": "h2. Heading\n\n*bold* and _italic_",
        }),
    )
    .await;

    let key = issue.key.clone().expect("a created issue carries a key");
    let read = server().issues().get_issue(&key).send().await.expect("the issue reads back by key");

    assert_eq!(read.key.as_deref(), Some(key.as_str()), "the issue read back is the one created");

    let description = read
        .fields
        .as_ref()
        .and_then(|fields| fields.get("description"))
        .and_then(serde_json::Value::as_str)
        .expect("a Data Center description is a plain string");

    assert!(description.contains("*bold*"), "the wiki markup arrives verbatim: {description}");

    let summary = test_name("renamed");

    server()
        .issues()
        .edit_issue(&key)
        .issue_update(IssueUpdate { fields: Some(fields(json!({ "summary": summary }))), ..IssueUpdate::default() })
        .send()
        .await
        .expect("a field can be edited");

    server()
        .issues()
        .assign(&key)
        .user(User { name: Some(username.clone()), ..User::default() })
        .send()
        .await
        .expect("an issue can be assigned by name, which is how Data Center addresses a user");

    let edited = server().issues().get_issue(&key).send().await.expect("the edited issue reads back");
    let edited = edited.fields.as_ref().expect("an issue carries its fields");

    assert_eq!(
        edited.get("summary").and_then(serde_json::Value::as_str),
        Some(summary.as_str()),
        "the edit is observable on the next read",
    );
    assert_eq!(
        edited.get("assignee").and_then(|assignee| assignee["name"].as_str()),
        Some(username.as_str()),
        "and so is the assignment",
    );

    // Read unmodelled on purpose: the document declares this paginated answer as a single issue type, so the
    // generated model has nowhere to put the page. The gap is the document's.
    let types = server()
        .issues()
        .get_create_issue_meta_project_issue_types(&project.key)
        .send_raw()
        .await
        .expect("the create metadata reads");

    assert!(types["values"].as_array().is_some_and(|values| !values.is_empty()), "{types}");

    // Read unmodelled for the same reason: `EditMeta` is declared with no properties at all.
    let meta = server().issues().get_edit_issue_meta(&key).send_raw().await.expect("the edit metadata reads");

    assert!(meta["fields"]["summary"].is_object(), "the summary is editable, and the metadata says so: {meta}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn creates_issues_in_bulk() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "bulk issues").await;

    let updates = ["bulk one", "bulk two"]
        .into_iter()
        .map(|label| IssueUpdate {
            fields: Some(fields(json!({
                "project": { "key": project.key },
                "issuetype": { "name": "Task" },
                "summary": test_name(label),
            }))),
            ..IssueUpdate::default()
        })
        .collect();

    let bulk = server()
        .issues()
        .create_issues()
        .issues_update(IssuesUpdate { issue_updates: Some(updates) })
        .send()
        .await
        .expect("two issues can be created in one request");

    let created = bulk.issues.as_ref().expect("a bulk creation answers with what it created");

    assert_eq!(created.len(), 2, "both issues were created");
    assert!(bulk.errors.as_ref().is_none_or(Vec::is_empty), "and neither of them failed: {:?}", bulk.errors);

    for issue in created {
        let key = issue.key.clone().expect("a created issue carries a key");

        assert!(key.starts_with(&format!("{}-", project.key)), "each key belongs to the project asked for: {key}");

        tracker.defer(move || {
            let key = key.clone();

            async move { server().issues().delete_issue(key).send().await }
        });
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn moves_the_issue_through_a_transition() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "transition subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to transition").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    let transitions = server().issues().get_transitions(&key).send().await.expect("the transitions read");
    let transitions = transitions.transitions.as_ref().expect("an open issue has transitions");

    assert!(!transitions.is_empty(), "a workflow offers somewhere to go");
    assert!(transitions.iter().all(|transition| transition.id.is_some()), "and every transition is addressable");

    let transition = transitions.first().expect("the first transition is the one taken").clone();
    let destination = transition.to.as_ref().and_then(|status| status.name.clone());

    server()
        .issues()
        .do_transition(&key)
        .issue_update(IssueUpdate {
            transition: Some(Transition { id: transition.id.clone(), ..Transition::default() }),
            ..IssueUpdate::default()
        })
        .send()
        .await
        .expect("an issue can be moved through a transition");

    let after = server().issues().get_issue(&key).fields("status").send().await.expect("the moved issue reads back");
    let status = after
        .fields
        .as_ref()
        .and_then(|fields| fields.get("status"))
        .and_then(|status| status["name"].as_str().map(ToOwned::to_owned))
        .expect("an issue always has a status");

    match destination {
        Some(destination) => assert_eq!(status, destination, "the issue is where the transition said it would go"),
        None => assert!(!status.is_empty(), "the issue names the status it landed in"),
    }

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn adds_updates_pins_and_deletes_a_comment() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "comment subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to comment on").await;
    let key = issue.key.clone().expect("a created issue carries a key");
    let body = test_name("a comment");

    let comment = server()
        .issues()
        .add_comment(&key)
        .comment_json(CommentJson { body: Some(body.clone()), ..CommentJson::default() })
        .send()
        .await
        .expect("a comment can be added");

    let id = comment.id.clone().expect("a created comment carries an id");

    assert_eq!(comment.body.as_deref(), Some(body.as_str()), "a comment is plain wiki markup on this surface");

    let updated = server()
        .issues()
        .update_comment(&key, &id)
        .body(CommentJson { body: Some(format!("{body} — edited")), ..CommentJson::default() })
        .send()
        .await
        .expect("a comment can be edited");

    assert!(updated.body.as_deref().is_some_and(|body| body.contains("edited")), "the edit is in the answer");

    let read = server().issues().get_comment(&key, &id).send().await.expect("the comment reads back by id");

    assert_eq!(read.id.as_deref(), Some(id.as_str()), "the comment read back is the one written");

    server().issues().set_pin_comment(&key, &id, true).send().await.expect("a comment can be pinned");

    let pinned = server().issues().get_pinned_comments(&key).send().await.expect("the pinned comments read");

    assert!(
        pinned
            .iter()
            .any(|entry| entry.comment.as_ref().and_then(|comment| comment.id.as_deref()) == Some(id.as_str())),
        "the comment just pinned is in the pinned listing",
    );

    server()
        .issue_comments()
        .set_comment_property("suite", &id, property_body())
        .send()
        .await
        .expect("a comment takes a property");

    let property =
        server().issue_comments().get_comment_property("suite", &id).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    server().issue_comments().delete_comment_property("suite", &id).send().await.expect("the property can be removed");
    server().issues().delete_comment(&key, &id).send().await.expect("the comment can be removed");

    let error = server().issues().get_comment(&key, &id).send().await.expect_err("a deleted comment cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn logs_work_and_edits_it() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "worklog subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to log work against").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    let worklog = server()
        .issues()
        .add_worklog(&key)
        .worklog(Worklog {
            time_spent: Some("2h".to_owned()),
            comment: Some(test_name("worklog")),
            ..Worklog::default()
        })
        .send()
        .await
        .expect("work can be logged");

    let id = worklog.id.clone().expect("a logged worklog carries an id");

    assert_eq!(worklog.time_spent.as_deref(), Some("2h"), "the time logged is the time asked for");

    server()
        .issues()
        .update_worklog(&key, &id)
        .body(Worklog { time_spent: Some("3h".to_owned()), ..Worklog::default() })
        .send()
        .await
        .expect("a worklog can be edited");

    let read = server().issues().get_worklog(&key, &id).send().await.expect("the worklog reads back");

    assert_eq!(read.time_spent.as_deref(), Some("3h"), "the edit is observable on the next read");

    let all = server().issues().get_issue_worklog(&key).send().await.expect("the worklogs of an issue read");

    assert!(
        all.worklogs.iter().flatten().any(|worklog| worklog.id.as_deref() == Some(id.as_str())),
        "the worklog just written is in the issue's listing",
    );

    let numeric: i64 = id.parse().expect("a worklog id is a number");
    let bulk = server()
        .issue_worklogs()
        .get_worklogs_for_ids(WorklogIdsRequest { ids: Some(vec![numeric]) })
        .send()
        .await
        .expect("worklogs can be fetched by id");

    assert!(bulk.iter().any(|worklog| worklog.id.as_deref() == Some(id.as_str())), "the bulk read finds it too");

    server().issues().delete_worklog(&key, &id).send().await.expect("a worklog can be removed");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn votes_and_watches() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "vote subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to vote on").await;
    let key = issue.key.clone().expect("a created issue carries a key");
    let username = admin_username();

    // The reporter cannot vote for their own issue, which is what makes this a `touch`: the request shape is what is
    // under test, and Jira refusing on those grounds is a correct answer.
    touch(server().issues().add_vote(&key).send().await);

    // Read unmodelled on purpose: the specification declares `Vote` and `Watchers` with no properties at all, so both
    // generated types are empty structs. The gap is the document's.
    let votes = server().issues().get_votes(&key).send_raw().await.expect("the votes read");

    assert!(votes["self"].as_str().is_some_and(|url| url.contains(&key)), "the votes belong to the issue: {votes}");

    touch(server().issues().remove_vote(&key).send().await);

    server().issues().add_watcher(&key).body(username.clone()).send().await.expect("a watcher can be added by name");

    let watchers = server().issues().get_issue_watchers(&key).send_raw().await.expect("the watchers read");

    assert!(
        watchers["watchCount"].as_i64().is_some_and(|count| count > 0),
        "the watcher just added is counted: {watchers}",
    );

    server().issues().remove_watcher(&key).username(username).send().await.expect("a watcher can be removed");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn stores_and_removes_a_property() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "issue property holder").await;
    let issue = create_task(&mut tracker, &project.key, "an issue with a property").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    server()
        .issues()
        .set_issue_property("suite", &key, property_body())
        .send()
        .await
        .expect("an issue takes a property of the caller's own");

    let property = server().issues().get_issue_property("suite", &key).send().await.expect("the property reads back");

    assert_eq!(property.value, Some(property_value()), "the value survives the round trip untouched");

    let keys = server().issues().get_issue_property_keys(&key).send().await.expect("the property keys read");

    assert!(
        keys.keys.iter().flatten().any(|entry| entry.key.as_deref() == Some("suite")),
        "the key just written is in the listing",
    );

    server().issues().delete_issue_property("suite", &key).send().await.expect("the property can be removed");

    let error = server()
        .issues()
        .get_issue_property("suite", &key)
        .send()
        .await
        .expect_err("a removed property cannot be read");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn keeps_remote_links_by_id_and_by_global_id() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "remote link holder").await;
    let issue = create_task(&mut tracker, &project.key, "an issue with remote links").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    // Read unmodelled on purpose: the specification declares `RemoteIssueLink` with no properties at all, so the
    // generated type is an empty struct and the id a caller needs to address the link never reaches them. The gap is
    // the document's; the body is what proves it is a gap rather than a limit of the client.
    let link = server()
        .issues()
        .create_or_update_remote_issue_link(&key)
        .remote_issue_link_create_or_update_request(remote_link("https://example.com/one", "one"))
        .send_raw()
        .await
        .expect("a remote link can be created");

    let link_id = link["id"].as_i64().expect("a created remote link carries an id").to_string();

    server()
        .issues()
        .update_remote_issue_link(&link_id, &key)
        .remote_issue_link_create_or_update_request(remote_link("https://example.com/two", "two"))
        .send()
        .await
        .expect("a remote link can be updated");

    let read = server()
        .issues()
        .get_remote_issue_link_by_id(&link_id, &key)
        .send_raw()
        .await
        .expect("a remote link reads back by id");

    assert_eq!(read["object"]["url"].as_str(), Some("https://example.com/two"), "the update is observable: {read}");

    let all =
        server().issues().get_remote_issue_links(&key).send_raw().await.expect("the remote links of an issue read");

    assert!(all.as_array().is_some_and(|links| !links.is_empty()), "the link just written is in the listing: {all}");

    server()
        .issues()
        .delete_remote_issue_link_by_id(&link_id, &key)
        .send()
        .await
        .expect("a remote link can be removed by id");

    server()
        .issues()
        .create_or_update_remote_issue_link(&key)
        .remote_issue_link_create_or_update_request(remote_link("https://example.com/three", "three"))
        .send_raw()
        .await
        .expect("and written again under the same global id");

    server()
        .issues()
        .delete_remote_issue_link_by_global_id(&key, REMOTE_LINK_GLOBAL_ID)
        .send()
        .await
        .expect("a remote link can be removed by its global id");

    // A reciprocal link wants an application on both ends of it, which a lone instance is not.
    touch(
        server()
            .issues()
            .create_reciprocal_remote_issue_link()
            .remote_reciprocal_issue_link_create_request(RemoteReciprocalIssueLinkCreateRequest {
                source: Some(remote_link("https://example.com/four", "four")),
                ..RemoteReciprocalIssueLinkCreateRequest::default()
            })
            .send()
            .await,
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn links_two_issues_and_unlinks_them() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "link holder").await;
    let issue = create_task(&mut tracker, &project.key, "one end of a link").await;
    let other = create_task(&mut tracker, &project.key, "the other end of a link").await;
    let key = issue.key.clone().expect("a created issue carries a key");
    let other_key = other.key.clone().expect("a created issue carries a key");

    server()
        .issue_links()
        .link_issues(LinkIssueRequestJson {
            r#type: Some(IssueLinkTypeJson { name: Some("Relates".to_owned()), ..IssueLinkTypeJson::default() }),
            inward_issue: Some(IssueRefJson { key: Some(key.clone()), ..IssueRefJson::default() }),
            outward_issue: Some(IssueRefJson { key: Some(other_key), ..IssueRefJson::default() }),
            ..LinkIssueRequestJson::default()
        })
        .send()
        .await
        .expect("two issues can be linked");

    let linked = server().issues().get_issue(&key).fields("issuelinks").send().await.expect("the issue reads back");
    let links = linked
        .fields
        .as_ref()
        .and_then(|fields| fields.get("issuelinks"))
        .and_then(serde_json::Value::as_array)
        .expect("the issuelinks field is a list");

    assert!(!links.is_empty(), "the link is on the issue that was linked");

    let link_id = links[0]["id"].as_str().expect("a link carries an id").to_owned();
    let read = server().issue_links().get_issue_link(&link_id).send().await.expect("the link reads back by id");

    assert_eq!(read.id.as_deref(), Some(link_id.as_str()), "the link read back is the one on the issue");
    assert_eq!(
        read.r#type.as_ref().and_then(|kind| kind.name.as_deref()),
        Some("Relates"),
        "and it is the kind of link that was asked for",
    );

    server().issue_links().delete_issue_link(&link_id).send().await.expect("a link can be removed");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn reads_and_moves_sub_tasks() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "sub-task holder").await;
    let parent = create_task(&mut tracker, &project.key, "a parent").await;
    let parent_key = parent.key.clone().expect("a created issue carries a key");

    create_issue(
        &mut tracker,
        json!({
            "project": { "key": project.key },
            "issuetype": { "name": "Sub-task" },
            "summary": test_name("a sub-task"),
            "parent": { "key": parent_key },
        }),
    )
    .await;

    let sub_tasks = server().issues().get_sub_tasks(&parent_key).send().await.expect("the sub-tasks of an issue read");

    assert_eq!(sub_tasks.len(), 1, "the parent has the one sub-task that was filed under it");

    let sub_task_id = sub_tasks[0].id.clone().expect("a sub-task carries an id");
    let can_move = server().issues().can_move_sub_task(&sub_task_id).send().await.expect("the move check answers");

    assert!(!can_move.is_null(), "a sub-task says whether it can be moved: {can_move}");

    // One sub-task cannot be reordered against itself, which is the refusal this proves is typed.
    touch(
        server()
            .issues()
            .move_sub_tasks(&parent_key, IssueSubTaskMovePosition { current: Some(0), original: Some(0) })
            .send()
            .await,
    );

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn attaches_a_file_and_removes_it() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "attachment holder").await;
    let issue = create_task(&mut tracker, &project.key, "an issue with an attachment").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    let attached = server()
        .issues()
        .add_attachment(&key, [jira::Attachment::new("suite.txt", "written by the issues suite")])
        .send()
        .await
        .expect("a file can be attached");

    let attachment = attached.first().expect("an upload answers with what it stored");
    let id = attachment.id.clone().expect("a stored attachment carries an id");

    assert_eq!(attachment.filename.as_deref(), Some("suite.txt"), "the file keeps the name it was uploaded under");

    // Read unmodelled on purpose: the specification declares `Attachment` with no properties at all, so the
    // generated type is an empty struct even though the upload's own answer is fully described. The gap is the
    // document's.
    let meta = server().issue_attachments().get_attachment(&id).send_raw().await.expect("the attachment reads back");

    assert_eq!(meta["filename"].as_str(), Some("suite.txt"), "and reads back under it: {meta}");
    assert_eq!(meta["size"].as_i64(), Some(27), "the bytes that went up are the bytes that are held: {meta}");

    server().issue_attachments().remove_attachment(&id).send().await.expect("an attachment can be removed");

    let error =
        server().issue_attachments().get_attachment(&id).send_raw().await.expect_err("a removed attachment is gone");

    assert!(error.is_not_found(), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn archives_restores_and_notifies() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "archive subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to archive").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    // Archiving needs Jira Software Data Center licensing that a timebomb does not always carry, and notifying needs
    // a mail server the rig has none of, so what is under test here is the request rather than the outcome.
    touch(server().issues().archive_issue(&key).send().await);
    touch(server().issues().restore_issue(&key).send().await);
    touch(server().issues().archive_issues().body(key.clone()).send().await);
    touch(
        server()
            .issues()
            .notify(&key)
            .notification_json(NotificationJson {
                subject: Some(test_name("a notification")),
                text_body: Some("sent by the Data Center suite".to_owned()),
                to: Some(ToJson { reporter: Some(true), ..ToJson::default() }),
                ..NotificationJson::default()
            })
            .send()
            .await,
    );

    let read = server().issues().get_issue(&key).send().await.expect("the issue survives being archived and restored");

    assert_eq!(read.key.as_deref(), Some(key.as_str()), "and it is still the issue it was");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jira-dc up`"]
async fn finds_the_issue_by_jql_and_through_the_picker() {
    if !software_licensed().await {
        return;
    }
    let mut tracker = ResourceTracker::new();
    let project = scrum_project(&mut tracker, "search subject").await;
    let issue = create_task(&mut tracker, &project.key, "an issue to find").await;
    let key = issue.key.clone().expect("a created issue carries a key");

    let found = poll_until("the issue to be indexed", || async {
        let page = server()
            .issue_search()
            .search_using_search_request(SearchRequest {
                jql: Some(format!("project = {}", project.key)),
                max_results: Some(5),
                fields: Some(vec!["summary".to_owned()]),
                ..SearchRequest::default()
            })
            .send()
            .await
            .expect("a search request body is accepted");

        page.issues.filter(|issues| !issues.is_empty())
    })
    .await;

    assert!(
        found.iter().any(|issue| issue.key.as_deref() == Some(key.as_str())),
        "the issue the suite created is what the project search finds",
    );

    let results = server()
        .issue_search()
        .search()
        .jql(format!("key = {key}"))
        .fields(SearchRequest2Fields::Many(vec!["summary".to_owned()]))
        .send()
        .await
        .expect("a JQL search is accepted as query parameters too");

    assert_eq!(
        results.issues.iter().flatten().filter_map(|issue| issue.key.as_deref()).collect::<Vec<_>>(),
        vec![key.as_str()],
        "a search by key finds exactly the issue named",
    );

    let picker = server()
        .issues()
        .get_issue_picker_resource()
        .query(key.clone())
        .send()
        .await
        .expect("the picker answers a query");

    assert!(
        picker.sections.as_ref().is_some_and(|sections| !sections.is_empty()),
        "the picker groups what it finds into sections",
    );

    tracker.cleanup().await;
}

/// A remote link to somewhere outside Jira, under the global id this suite writes its links with.
fn remote_link(url: &str, title: &str) -> RemoteIssueLinkCreateOrUpdateRequest {
    RemoteIssueLinkCreateOrUpdateRequest {
        global_id: Some(REMOTE_LINK_GLOBAL_ID.to_owned()),
        object: Some(RemoteObject {
            url: Some(url.to_owned()),
            title: Some(title.to_owned()),
            ..RemoteObject::default()
        }),
        ..RemoteIssueLinkCreateOrUpdateRequest::default()
    }
}
