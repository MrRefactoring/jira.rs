use jira::cloud::{SearchWorkflowsRequestExpand, SearchWorkflowsRequestExpandValue, WorkflowSearchResponse};

use crate::harness::{TEST_PROJECT_KEY, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_workflow_listing_for_an_admin_or_fails_typed() {
    let Some(page) = search_workflows(5).await else { return };

    assert_eq!(page.max_results, Some(5), "the page size asked for is the page size returned");

    for workflow in page.values.as_deref().unwrap_or_default() {
        assert!(workflow.id.as_ref().is_some_and(|id| !id.is_empty()), "a workflow carries an id");
        assert!(workflow.name.as_ref().is_some_and(|name| !name.is_empty()), "a workflow carries a name");
        assert!(workflow.description.is_some(), "a workflow carries a description, and it is a plain string");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_transitions_only_when_expand_asks_for_them() {
    let Some(plain) = search_workflows(1).await else { return };

    let expanded = cloud()
        .workflows()
        .search_workflows()
        .max_results(1)
        .expand(SearchWorkflowsRequestExpand::One(SearchWorkflowsRequestExpandValue::ValuesTransitions))
        .send()
        .await
        .expect("the expand parameter is accepted");

    let unexpanded_transitions = plain
        .values
        .as_deref()
        .unwrap_or_default()
        .first()
        .and_then(|workflow| workflow.transitions.as_deref())
        .unwrap_or_default();

    assert!(unexpanded_transitions.is_empty(), "an unexpanded workflow carries no transitions");

    let expanded_transitions = expanded
        .values
        .as_deref()
        .unwrap_or_default()
        .first()
        .and_then(|workflow| workflow.transitions.as_deref())
        .unwrap_or_default();

    assert!(!expanded_transitions.is_empty(), "an expanded workflow carries the transitions it allows");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_each_transition_with_a_type() {
    if search_workflows(1).await.is_none() {
        return;
    }

    let page = cloud()
        .workflows()
        .search_workflows()
        .max_results(1)
        .expand(SearchWorkflowsRequestExpand::One(SearchWorkflowsRequestExpandValue::ValuesTransitions))
        .send()
        .await
        .expect("the expand parameter is accepted");

    let transitions = page
        .values
        .as_deref()
        .unwrap_or_default()
        .first()
        .and_then(|workflow| workflow.transitions.as_deref())
        .unwrap_or_default();

    assert!(!transitions.is_empty(), "an expanded workflow carries the transitions it allows");

    for transition in transitions {
        assert!(transition.name.as_ref().is_some_and(|name| !name.is_empty()), "a transition carries a name");

        let kind = transition.r#type.as_ref().expect("a transition carries a type");

        assert!(
            matches!(kind.as_str(), "INITIAL" | "GLOBAL" | "DIRECTED"),
            "a transition type is one of the three the API documents, got {}",
            kind.as_str(),
        );
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_the_projects_a_workflow_is_used_by() {
    let Some(page) = search_workflows(1).await else { return };

    let Some(workflow_id) = page.values.as_deref().unwrap_or_default().first().and_then(|workflow| workflow.id.clone())
    else {
        return;
    };

    match cloud().workflows().get_project_usages_for_workflow(workflow_id.as_str()).send().await {
        Ok(usages) => assert_eq!(
            usages.workflow_id.as_deref(),
            Some(workflow_id.as_str()),
            "the usage report names the workflow it was asked about",
        ),
        Err(error) => {
            assert!(
                error.status().is_some_and(|status| (400..500).contains(&status)),
                "a refused usage report is typed: {error}"
            );
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_capabilities_available_when_authoring_a_workflow() {
    let capabilities = match cloud().workflows().workflow_capabilities().send().await {
        Ok(capabilities) => capabilities,
        Err(error) => {
            assert!(
                error.status().is_some_and(|status| (400..500).contains(&status)),
                "a refused listing is typed: {error}"
            );

            return;
        }
    };

    if let Some(scope) = &capabilities.editor_scope {
        assert!(
            matches!(scope.as_str(), "GLOBAL" | "PROJECT"),
            "the editor is scoped globally or to a project, got {}",
            scope.as_str(),
        );
    }

    for rule in capabilities.system_rules.as_deref().unwrap_or_default() {
        assert!(!rule.rule_key.is_empty(), "an Atlassian-provided rule carries a key");
        assert!(!rule.name.is_empty(), "an Atlassian-provided rule carries a name");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn shows_which_workflow_the_test_project_resolves_to() {
    let issue_types = cloud()
        .projects()
        .get_all_statuses(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project lists its statuses");

    assert!(!issue_types.is_empty(), "the test project carries at least one issue type");

    let first = issue_types.first().expect("the listing is not empty");

    assert!(!first.statuses.is_empty(), "an issue type resolves to the statuses its workflow allows");
    assert!(
        first.statuses.iter().all(|status| status.name.as_ref().is_some_and(|name| !name.is_empty())),
        "every status the workflow allows carries a name",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .workflows()
        .delete_inactive_workflow("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .expect_err("a workflow that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

async fn search_workflows(max_results: i64) -> Option<WorkflowSearchResponse> {
    match cloud().workflows().search_workflows().max_results(max_results).send().await {
        Ok(page) => Some(page),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            None
        }
    }
}
