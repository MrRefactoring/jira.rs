use jira::server::IssueUpdateDetails;
use jira::service_desk_server::{
    ApprovalDecisionRequest, ApprovalDecisionRequestDecision, ApprovalFinalDecision, CustomerCreate, RequestCreate,
    ServiceDeskCustomerAdd,
};
use serde_json::json;

use super::fixtures::{asset_name, service_desk_licensed, service_desk_project};
use crate::harness::{ResourceTracker, jsm_platform, poll_until, run_id, service_desk_server};

#[tokio::test]
#[ignore = "live: needs `cargo xtask jsm-dc up`"]
async fn reads_a_pending_approval_answers_it_and_checks_the_decision() {
    if !service_desk_licensed().await {
        return;
    }

    let project = service_desk_project().await;
    let desk = project.service_desk_id.to_string();
    let me = jsm_platform().myself().get_current_user().send().await.expect("the administrator identifies itself");
    let approver = me.name.expect("the administrator carries a username");
    let fields = jsm_platform().issue_fields().get_fields().send().await.expect("the platform lists its fields");
    let approver_field = fields
        .into_iter()
        .find(|field| field.name.as_deref() == Some("Approvers"))
        .and_then(|field| field.id)
        .expect("the service desk template creates an Approvers field");
    let types = service_desk_server()
        .request_types()
        .get_request_types(desk.clone())
        .send()
        .await
        .expect("the service desk lists its request types");
    let mut approval_type = None;

    for kind in types.values {
        let id = kind.id.expect("a template request type carries an id");
        let form = service_desk_server()
            .request_types()
            .get_request_type_fields(desk.clone(), id.clone())
            .send()
            .await
            .expect("a template request type describes its form");

        if form
            .request_type_fields
            .unwrap_or_default()
            .iter()
            .any(|field| field.field_id.as_ref() == Some(&approver_field))
        {
            approval_type = Some(id);
            break;
        }
    }

    let approval_type = approval_type.expect("the service desk template provides an approval request type");
    let mut tracker = ResourceTracker::new();
    let customer = service_desk_server()
        .customers()
        .create_customer()
        .customer_create(CustomerCreate {
            email: Some(format!("jirars-{}-approval@example.invalid", run_id())),
            full_name: Some(asset_name("approval customer")),
        })
        .send()
        .await
        .expect("the instance creates the approval customer");
    let customer = customer.name.expect("a created customer carries a username");
    let doomed = customer.clone();
    tracker.defer(move || {
        let doomed = doomed.clone();
        async move { jsm_platform().users().remove_user().username(doomed).send().await }
    });
    assert_ne!(customer, approver, "the approver is not the reporter");
    service_desk_server()
        .customers()
        .add_customers(desk.clone())
        .service_desk_customer_add(ServiceDeskCustomerAdd { usernames: Some(vec![customer.clone()]) })
        .send()
        .await
        .expect("the customer can raise requests in the service desk");
    let request = service_desk_server()
        .customer_requests()
        .create_customer_request()
        .request_create(RequestCreate {
            service_desk_id: Some(desk),
            request_type_id: Some(approval_type),
            raise_on_behalf_of: Some(customer),
            request_field_values: Some(
                [
                    ("summary".to_owned(), json!(asset_name("approval request"))),
                    ("description".to_owned(), json!("Created by the approval live test.")),
                ]
                .into_iter()
                .collect(),
            ),
            ..RequestCreate::default()
        })
        .send()
        .await
        .expect("the service desk raises an approval request");
    let key = request.issue_key.expect("a created approval request carries an issue key");
    let doomed = key.clone();
    tracker.defer(move || {
        let doomed = doomed.clone();
        async move { jsm_platform().issues().delete_issue(doomed).send().await }
    });
    let before = request.current_status.and_then(|status| status.status).expect("the request carries a status");
    jsm_platform()
        .issues()
        .edit_issue(
            key.clone(),
            IssueUpdateDetails {
                fields: Some([(approver_field, json!([{ "name": approver }]))].into_iter().collect()),
                ..IssueUpdateDetails::default()
            },
        )
        .send()
        .await
        .expect("the request accepts its approver");
    let pending = poll_until("the request offers its approver a pending approval", || async {
        service_desk_server()
            .approvals()
            .get_approvals(key.clone())
            .send()
            .await
            .expect("the request lists its approvals")
            .values
            .into_iter()
            .find(|approval| approval.can_answer_approval == Some(true))
    })
    .await;
    let id = pending.id.expect("a pending approval carries an id");
    let pending = service_desk_server()
        .approvals()
        .get_approval_by_id(key.clone(), id.clone())
        .send()
        .await
        .expect("a pending approval reads back by id");
    assert_eq!(pending.final_decision, Some(ApprovalFinalDecision::Pending));
    assert_eq!(pending.can_answer_approval, Some(true));
    let config = service_desk_server()
        .approvals()
        .get_approval_comment_config(key.clone(), id.clone())
        .send()
        .await
        .expect("the approval describes its comment rules");
    assert!(config.comments_required_when_approve.is_some());
    assert!(config.comments_required_when_decline.is_some());
    let answered = service_desk_server()
        .approvals()
        .answer_approval(key.clone(), id.clone())
        .approval_decision_request(ApprovalDecisionRequest {
            decision: Some(ApprovalDecisionRequestDecision::Approve),
            comment: Some(asset_name("approval comment")),
            comment_public: Some(true),
        })
        .send()
        .await
        .expect("the approver can approve the pending request");
    assert_eq!(answered.final_decision, Some(ApprovalFinalDecision::Approved));
    let read = service_desk_server()
        .approvals()
        .get_approval_by_id(key.clone(), id)
        .send()
        .await
        .expect("the approval decision reads back");
    assert_eq!(read.final_decision, Some(ApprovalFinalDecision::Approved));
    assert_eq!(read.can_answer_approval, Some(false));
    poll_until("approval moves the request to its next status", || async {
        let after = service_desk_server()
            .customer_requests()
            .get_customer_request_by_id_or_key(key.clone())
            .send()
            .await
            .expect("the approved request reads back")
            .current_status
            .and_then(|status| status.status)
            .expect("the approved request carries a status");
        (after != before).then_some(after)
    })
    .await;
    tracker.cleanup().await;
}
