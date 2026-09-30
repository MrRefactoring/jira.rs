use jira::cloud::{
    ForgePanelProjectPinRequest, GetWorkflowTransitionRuleConfigurationsRequestTypes as RuleType, Id,
    PublishDraftWorkflowScheme,
};

use crate::harness::{TEST_PROJECT_KEY, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_no_draft_for_a_scheme_that_has_never_been_edited() {
    let Some(scheme_id) = first_scheme_id().await else { return };

    match cloud().workflow_scheme_drafts().get_workflow_scheme_draft(scheme_id).send().await {
        Ok(draft) => {
            assert!(draft.name.as_ref().is_some_and(|name| !name.is_empty()), "a draft carries the scheme's name");
        }
        Err(error) => assert!(error.is_not_found(), "an unedited scheme has no draft to read: {error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_a_draft_lookup_on_an_unknown_scheme_as_a_typed_error() {
    let error = cloud()
        .workflow_scheme_drafts()
        .get_workflow_scheme_draft(99_999_999)
        .send()
        .await
        .expect_err("a scheme that does not exist has no draft");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_publishing() {
    let error = cloud()
        .workflow_scheme_drafts()
        .publish_draft_workflow_scheme(99_999_999, PublishDraftWorkflowScheme { status_mappings: Some(Vec::new()) })
        .send()
        .await
        .expect_err("a draft that does not exist cannot be published");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_app_owned_transition_rule_reads() {
    let error = cloud()
        .workflow_transition_rules()
        .get_workflow_transition_rule_configurations([RuleType::Postfunction])
        .max_results(5)
        .send()
        .await
        .expect_err("only an app may read the transition rules it owns");

    assert!(
        error.status().is_some_and(|status| (400..500).contains(&status)),
        "an app-only endpoint refuses a site token with a client error: {error}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_permission_scheme_assigned_to_the_test_project() {
    let assigned = cloud().project_permission_schemes().get_assigned_permission_scheme(TEST_PROJECT_KEY).send().await;

    let scheme = match assigned {
        Ok(scheme) => scheme,
        Err(error) => {
            assert!(error.is_forbidden() || error.is_not_found(), "a refused permission scheme read is typed: {error}");

            return;
        }
    };

    assert!(scheme.id.is_some_and(|id| id > 0), "an assigned permission scheme carries an id");
    assert!(!scheme.name.is_empty(), "an assigned permission scheme carries a name");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_reassigning_a_permission_scheme() {
    let error = cloud()
        .project_permission_schemes()
        .assign_permission_scheme(TEST_PROJECT_KEY, Id { id: 99_999_999 })
        .send()
        .await
        .expect_err("a permission scheme that does not exist cannot be assigned");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn queues_the_asynchronous_issue_panel_write_returning_the_task_that_tracks_it() {
    let queued = cloud()
        .issue_panels()
        .bulk_pin_unpin_projects_async(ForgePanelProjectPinRequest {
            module_id: "absent-module".to_owned(),
            project_list: Vec::new(),
        })
        .send()
        .await
        .expect("the panel write is queued");

    let task_id = queued.task_id.as_deref().expect("a queued write carries the id of the task tracking it");

    assert!(!task_id.is_empty() && task_id.chars().all(|c| c.is_ascii_digit()), "a task id is digits: {task_id}");
}

async fn first_scheme_id() -> Option<i64> {
    match cloud().workflow_schemes().get_all_workflow_schemes().max_results(1).send().await {
        Ok(page) => page.values.first().and_then(|scheme| scheme.id),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            None
        }
    }
}
