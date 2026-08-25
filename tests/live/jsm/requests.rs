use std::collections::HashMap;

use jira::service_desk_server::{
    AttachmentCreate, CommentCreate, CustomerTransitionExecution, RequestCreate, RequestParticipantUpdate,
};
use serde_json::json;

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::service_desk_server;

async fn raise_a_request(label: &str) -> String {
    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();

    let types = service_desk_server()
        .request_types()
        .get_request_types(desk.clone())
        .send()
        .await
        .expect("a service desk lists its request types");

    let request_type_id =
        types.values.first().and_then(|kind| kind.id.clone()).expect("the template brings request types with it");

    let fields: HashMap<String, serde_json::Value> = [
        ("summary".to_owned(), json!(asset_name(label))),
        ("description".to_owned(), json!("Raised by the live suite.")),
    ]
    .into_iter()
    .collect();

    let created = service_desk_server()
        .customer_requests()
        .create_customer_request()
        .request_create(RequestCreate {
            service_desk_id: Some(desk),
            request_type_id: Some(request_type_id),
            request_field_values: Some(fields),
            ..RequestCreate::default()
        })
        .send()
        .await
        .expect("a service desk accepts a customer request");

    created.issue_key.expect("a raised request carries an issue key")
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn raises_a_request_reads_it_back_and_lists_the_callers_own() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("request").await;

    let loaded = service_desk_server()
        .customer_requests()
        .get_customer_request_by_id_or_key(key.clone())
        .send()
        .await
        .expect("a customer request reads back by key");

    assert_eq!(loaded.issue_key.as_deref(), Some(key.as_str()), "the request read back is the one raised");

    let status = service_desk_server()
        .customer_requests()
        .get_customer_request_status(key.clone())
        .send()
        .await
        .expect("a customer request reports its status");

    assert!(!status.values.is_empty(), "a request that exists has been in at least one status");

    let mine = service_desk_server()
        .customer_requests()
        .get_my_customer_requests()
        .send()
        .await
        .expect("the caller lists the requests it raised");

    assert!(
        mine.values.iter().any(|request| request.issue_key.as_deref() == Some(key.as_str())),
        "the request just raised is among the caller's own",
    );
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn comments_on_a_request_and_reads_the_comment_back() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("commented request").await;

    let created = service_desk_server()
        .customer_requests()
        .create_request_comment(key.clone())
        .comment_create(CommentCreate { body: Some(asset_name("comment")), public: Some(true) })
        .send()
        .await
        .expect("a customer request accepts a comment");

    let id = created.id.clone().expect("a created comment carries an id");

    let listed = service_desk_server()
        .customer_requests()
        .get_request_comments(key.clone())
        .send()
        .await
        .expect("a customer request lists its comments");

    assert!(listed.values.iter().any(|comment| comment.id == created.id), "the comment just written is among them");

    let loaded = service_desk_server()
        .customer_requests()
        .get_request_comment_by_id(key, id)
        .send()
        .await
        .expect("a comment reads back by id");

    assert_eq!(loaded.id, created.id, "the comment read back is the one asked for");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn adds_lists_and_removes_the_participants_of_a_request() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("shared request").await;

    let listed = service_desk_server()
        .customer_requests()
        .get_request_participants(key.clone())
        .send()
        .await
        .expect("a customer request lists its participants");

    let _ = listed;

    service_desk_server()
        .customer_requests()
        .add_request_participants(key.clone())
        .request_participant_update(RequestParticipantUpdate { usernames: Some(vec!["admin".to_owned()]) })
        .send()
        .await
        .expect("a customer request accepts a participant");

    service_desk_server()
        .customer_requests()
        .remove_request_participants(key)
        .request_participant_update(RequestParticipantUpdate { usernames: Some(vec!["admin".to_owned()]) })
        .send()
        .await
        .expect("a participant can be removed from a customer request");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_the_service_level_agreements_of_a_request() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("timed request").await;

    let mut id = None;

    for _ in 0..10 {
        let slas = service_desk_server()
            .customer_requests()
            .get_sla_information(key.clone())
            .send()
            .await
            .expect("a customer request reports the agreements it is measured against");

        id = slas.values.first().and_then(|sla| sla.id.clone());

        if id.is_some() {
            break;
        }

        tokio::time::sleep(std::time::Duration::from_millis(400)).await;
    }

    let id = id.expect("the agreements of a raised request are computed shortly after it is raised");

    let one = service_desk_server()
        .customer_requests()
        .get_sla_information_by_id(key, id.clone())
        .send()
        .await
        .expect("one agreement reads back by id");

    assert_eq!(one.id, Some(id), "the agreement read back is the one asked for");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn lists_and_performs_the_transitions_a_customer_may_make() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("transitioning request").await;

    let transitions = service_desk_server()
        .customer_transitions()
        .get_customer_transitions(key.clone())
        .send()
        .await
        .expect("a customer request lists the transitions its customer may make");

    assert!(transitions.values.iter().all(|transition| transition.id.is_some()), "every transition carries an id");

    let id = transitions
        .values
        .first()
        .and_then(|transition| transition.id.clone())
        .expect("the workflow offers the customer at least one transition");

    service_desk_server()
        .customer_transitions()
        .perform_customer_transition(key)
        .customer_transition_execution(CustomerTransitionExecution { id: Some(id), additional_comment: None })
        .send()
        .await
        .expect("a customer may make the transition the workflow offers");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reports_the_approvals_of_a_request() {
    if !service_desk_licensed().await {
        return;
    }

    let key = raise_a_request("approved request").await;

    let approvals = service_desk_server()
        .approvals()
        .get_approvals(key)
        .send()
        .await
        .expect("a customer request reports the approvals hung off it");

    assert!(approvals.values.iter().all(|approval| approval.id.is_some()), "every approval carries an id");
}

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn uploads_a_temporary_file_and_attaches_it_to_a_request() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let key = raise_a_request("attached request").await;

    let uploaded = service_desk_server()
        .request_attachments()
        .attach_temporary_file(
            project.service_desk_id.to_string(),
            [jira::Attachment::new("jira-rs.txt", &b"attached by the live suite"[..])],
        )
        .send()
        .await
        .expect("a service desk accepts a temporary file");

    let id = uploaded
        .temporary_attachments
        .unwrap_or_default()
        .into_iter()
        .find_map(|attachment| attachment.temporary_attachment_id)
        .expect("an uploaded file is given a temporary id");

    service_desk_server()
        .request_attachments()
        .create_attachment(key)
        .attachment_create(AttachmentCreate {
            temporary_attachment_ids: Some(vec![id]),
            public: Some(true),
            additional_comment: None,
        })
        .send()
        .await
        .expect("a temporary file can be attached to a request");
}
