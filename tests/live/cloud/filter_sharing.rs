//! Ported from jira.js/tests/live/cloud/filterSharing.test.ts.
//!
//! The sharing cycle runs in full against a filter this suite creates, so nothing anyone else owns is ever shared.
//! That matters more here than elsewhere: sharing is the one write in this file whose effect is that *other people*
//! can see something they could not before.
//!
//! `set_default_share_scope` is deliberately left alone. It is per-account rather than per-filter, and it decides
//! whether every filter the account creates in future is private or global — a standing preference, not a scoped
//! change. The neighbouring resolution and project-email reads sit here because they are the two other reads a
//! filter's owner reaches for, and neither is large enough to earn a file.

use jira::cloud::{Filter, SharePermissionInput, SharePermissionInputType, SharePermissionType};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, await_readable, cloud, poll_until, test_name};

/// The sharing cycle, end to end.
///
/// A share permission is reachable three ways — the listing, the single read, and the filter itself — and all three
/// have to agree, because a caller that trusts one of them has to be able to trust the rest.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn shares_a_filter_with_one_project_and_takes_it_back() {
    let mut tracker = ResourceTracker::new();
    let project_id = test_project_id().await;
    let filter_id = create_filter(&mut tracker).await;

    let fresh = cloud()
        .filter_sharing()
        .get_share_permissions(filter_id)
        .send()
        .await
        .expect("a new filter lists its share permissions");

    assert!(fresh.is_empty(), "a filter this suite just created is shared with nobody");

    let added = cloud()
        .filter_sharing()
        .add_share_permission(
            filter_id,
            SharePermissionInput {
                account_id: None,
                group_id: None,
                groupname: None,
                project_id: Some(project_id.clone()),
                project_role_id: None,
                rights: None,
                r#type: SharePermissionInputType::Project,
            },
        )
        .send()
        .await
        .expect("the filter can be shared with the test project");

    assert!(!added.is_empty(), "adding a share permission answers with what the filter is now shared with");

    let permissions = await_readable("the share reads back through the sharing API", || {
        cloud().filter_sharing().get_share_permissions(filter_id).send()
    })
    .await;

    assert_eq!(permissions.len(), 1, "one share was added, so one is listed");
    assert_eq!(permissions[0].r#type, SharePermissionType::Project);
    assert_eq!(
        permissions[0].project.as_ref().and_then(|project| project.id.as_deref()),
        Some(project_id.as_str()),
        "the filter is shared with the project it was given, not another one",
    );

    let permission_id = permissions[0].id.expect("a stored share permission carries an id");

    let single = cloud()
        .filter_sharing()
        .get_share_permission(filter_id, permission_id)
        .send()
        .await
        .expect("one share permission reads back by id");

    assert_eq!(single.id, Some(permission_id));
    assert_eq!(single.r#type, SharePermissionType::Project, "reading one permission gives the record the listing did");

    let filter = cloud().filters().get_filter(filter_id).send().await.expect("the shared filter reads back");

    assert_eq!(
        filter.share_permissions.as_deref().unwrap_or_default().len(),
        1,
        "the share is visible on the filter itself, not only through the sharing API",
    );

    cloud()
        .filter_sharing()
        .delete_share_permission(filter_id, permission_id)
        .send()
        .await
        .expect("the share can be taken back");

    poll_until("unsharing to return the filter to private", || async {
        let remaining = cloud()
            .filter_sharing()
            .get_share_permissions(filter_id)
            .send()
            .await
            .expect("the listing reads after an unshare");

        remaining.is_empty().then_some(())
    })
    .await;

    tracker.cleanup().await;
}

/// Read, never written: the scope decides what every filter the account creates in future defaults to.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_account_default_share_scope_without_changing_it() {
    let scope = cloud()
        .filter_sharing()
        .get_default_share_scope()
        .send()
        .await
        .expect("the account reports the scope new filters default to");

    assert!(scope.scope.is_documented(), "the default scope is one the specification names: {}", scope.scope.as_str());
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_sharing_of_an_unknown_filter_as_not_found() {
    let error = cloud()
        .filter_sharing()
        .get_share_permissions(99_999_999)
        .send()
        .await
        .expect_err("a filter that does not exist has no share permissions");

    assert!(error.is_not_found(), "{error}");
}

/// A resolution the site actually uses where there is one, and Jira's own first id otherwise.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_an_issue_resolution_by_id() {
    let resolution_id = cloud()
        .issues()
        .get_issue(format!("{TEST_PROJECT_KEY}-1"))
        .send()
        .await
        .ok()
        .and_then(|issue| issue.fields.and_then(|fields| fields.resolution))
        .and_then(|resolution| resolution.id)
        .unwrap_or_else(|| "10000".to_owned());

    match cloud().issue_resolutions().get_resolution(resolution_id.as_str()).send().await {
        Ok(resolution) => {
            assert_eq!(resolution.id.as_deref(), Some(resolution_id.as_str()));
            assert!(resolution.name.as_deref().is_some_and(|name| !name.is_empty()), "{resolution:?}");
        }
        // A site that has never configured resolutions need not define the id Jira ships with.
        Err(error) => assert!(error.is_not_found(), "a resolution that does not exist is typed as missing: {error}"),
    }
}

/// The sender address a project emails from — configured on some sites, refused on others, typed on both.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_project_email_address_or_refuses_typed() {
    let project_id: i64 = test_project_id().await.parse().expect("a project id is a number");

    match cloud().project_email().get_project_email(project_id).send().await {
        Ok(email) => assert!(
            email.email_address.as_deref().is_some_and(|address| !address.is_empty()),
            "a project that answers names the address it sends from: {email:?}",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "a refusal is typed: {error}"),
    }
}

async fn test_project_id() -> String {
    cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back by key")
        .id
        .expect("a project carries an id")
}

/// A filter is owned by the account that created it and is private until shared, which is what makes it safe to
/// share here: the only thing this suite can widen access to is something it made a moment earlier.
async fn create_filter(tracker: &mut ResourceTracker) -> i64 {
    let filter = cloud()
        .filters()
        .create_filter(Filter {
            name: test_name("sharing"),
            jql: Some(format!("project = {TEST_PROJECT_KEY}")),
            ..Filter::default()
        })
        .send()
        .await
        .expect("the account may create a filter of its own");

    let id: i64 = filter.id.as_deref().expect("a created filter has an id").parse().expect("a filter id is a number");

    tracker.defer(move || async move { cloud().filters().delete_filter(id).send().await });

    poll_until("the filter just created to read back", || async { cloud().filters().get_filter(id).send().await.ok() })
        .await;

    id
}
