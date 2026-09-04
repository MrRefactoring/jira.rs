use crate::harness::{
    ResourceTracker, TEST_PROJECT_KEY, await_readable, await_refused, cloud, create_test_issue, test_name,
};
use jira::cloud::{
    GetProjectComponentsPaginatedRequestOrderBy, IssueFields, IssueUpdateDetails, ProjectComponent,
    ProjectComponentAssigneeType,
};

/// A component, from creation to deletion, inside the standing test project.
///
/// Components are project-scoped and deletable by their creator, so unlike most Jira configuration they can be
/// exercised end to end without leaving anything behind. The part that needs a live site is deletion semantics: a
/// component can be attached to issues, and removing it has to leave those issues intact. Nothing in the types
/// says so.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_component_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let name = test_name("component").replace(['[', ']'], "");

    let created = cloud()
        .project_components()
        .create_component(ProjectComponent {
            name: Some(name.clone()),
            description: Some("created by the live suite".to_owned()),
            project: Some(TEST_PROJECT_KEY.to_owned()),
            assignee_type: Some(ProjectComponentAssigneeType::ProjectDefault),
            ..ProjectComponent::default()
        })
        .send()
        .await
        .expect("the test project accepts a new component");

    let component_id = created.id.clone().expect("a created component carries an id");

    {
        let id = component_id.clone();

        tracker.defer(move || {
            let id = id.clone();

            async move { cloud().project_components().delete_component(id).send().await }
        });
    }

    assert!(component_id.chars().all(|c| c.is_ascii_digit()), "an id is digits: {component_id}");
    assert_eq!(created.name.as_deref(), Some(name.as_str()));
    assert_eq!(created.project.as_deref(), Some(TEST_PROJECT_KEY));
    assert!(created.self_.is_some_and(|link| link.starts_with("https://")), "a component carries a self link");

    let read = await_readable("the component reads back by id", || {
        cloud().project_components().get_component(&component_id).send()
    })
    .await;

    assert_eq!(read.id.as_deref(), Some(component_id.as_str()));
    assert_eq!(read.description.as_deref(), Some("created by the live suite"));
    assert_eq!(read.assignee_type, Some(ProjectComponentAssigneeType::ProjectDefault));
    assert_eq!(read.is_assignee_type_valid, Some(true));

    let all = cloud()
        .project_components()
        .get_project_components(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project lists its components");

    assert!(
        all.iter().any(|component| component.id.as_deref() == Some(component_id.as_str())),
        "the new component is among the project components",
    );

    cloud()
        .project_components()
        .update_component(
            &component_id,
            ProjectComponent { description: Some("edited".to_owned()), ..ProjectComponent::default() },
        )
        .send()
        .await
        .expect("the description can be edited");

    let after_edit = await_readable("the edited component reads back", || {
        cloud().project_components().get_component(&component_id).send()
    })
    .await;

    assert_eq!(after_edit.description.as_deref(), Some("edited"), "the edit is observable on the next read");
    assert_eq!(after_edit.name.as_deref(), Some(name.as_str()), "editing the description leaves the name alone");

    let issue = create_test_issue(&mut tracker, Some(&test_name("component holder"))).await;

    cloud()
        .issues()
        .edit_issue(
            &issue.key,
            IssueUpdateDetails {
                fields: Some(IssueFields {
                    components: Some(vec![ProjectComponent {
                        id: Some(component_id.clone()),
                        ..ProjectComponent::default()
                    }]),
                    ..IssueFields::default()
                }),
                ..IssueUpdateDetails::default()
            },
        )
        .send()
        .await
        .expect("the component can be attached to an issue");

    let related = cloud()
        .project_components()
        .get_component_related_issues(&component_id)
        .send()
        .await
        .expect("the component reports the issues attached to it");

    assert_eq!(related.issue_count, Some(1));

    let found = cloud()
        .project_components()
        .find_components_for_projects()
        .project_ids_or_keys([TEST_PROJECT_KEY])
        .query(&name)
        .send()
        .await
        .expect("the cross-project component search is accepted");

    assert!(
        found.values.iter().any(|component| component.id.as_deref() == Some(component_id.as_str())),
        "the cross-project search finds the component by name",
    );

    let collision = cloud()
        .project_components()
        .create_component(ProjectComponent {
            name: Some(name.clone()),
            project: Some(TEST_PROJECT_KEY.to_owned()),
            ..ProjectComponent::default()
        })
        .send()
        .await
        .expect_err("two components in one project cannot share a name");

    assert_eq!(collision.status(), Some(400), "{collision}");

    cloud().project_components().delete_component(&component_id).send().await.expect("the component can be deleted");

    let gone = await_refused("a deleted component cannot be read", || {
        cloud().project_components().get_component(&component_id).send()
    })
    .await;

    assert!(gone.is_not_found(), "{gone}");

    let fetched = await_readable("the issue that carried the component reads back", || {
        cloud().issues().get_issue(&issue.key).fields(["components"]).send()
    })
    .await;

    let components =
        fetched.fields.and_then(|fields| fields.components).expect("an issue reports its components as a list");

    assert!(components.is_empty(), "deleting the component leaves the issue intact and unattached");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_and_orders_the_paginated_component_listing() {
    let page = cloud()
        .project_components()
        .get_project_components_paginated(TEST_PROJECT_KEY)
        .max_results(1)
        .order_by(GetProjectComponentsPaginatedRequestOrderBy::Name)
        .send()
        .await
        .expect("the paginated component listing is accepted");

    assert!(page.values.len() <= 1, "one result was asked for, {} arrived", page.values.len());
    assert_eq!(page.max_results, 1);
    assert!(page.total >= page.values.len() as i64, "a page never reports fewer results than it carries");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_component_as_not_found() {
    let error = cloud()
        .project_components()
        .get_component("99999999")
        .send()
        .await
        .expect_err("a component that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}
