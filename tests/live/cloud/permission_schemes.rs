use jira::cloud::{GetPermissionSchemeRequestExpand, GetPermissionSchemeRequestExpandValue};

use crate::harness::{TEST_PROJECT_KEY, cloud};

/// The permission schemes API, read-only and emphatically so.
///
/// A permission scheme is shared by every project attached to it, so editing one silently changes who can do what
/// across projects this suite knows nothing about. Adding a grant is how you accidentally give a group rights it
/// should not have; removing one is how you lock people out. Nothing here creates, attaches or deletes a scheme.
///
/// The read half earns its place by explaining the rest of the suites. Every write the live tests perform is possible
/// because a scheme grants a permission to a project role the account happens to sit in — this file is what makes
/// that chain inspectable rather than folklore, and it is the first place to look when a write suite starts failing
/// with 403s that were not there yesterday.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_which_scheme_the_test_project_is_attached_to() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    assert!(scheme_id > 0, "a scheme id is a positive number, got {scheme_id}");

    let scheme = cloud()
        .permission_schemes()
        .get_permission_scheme(scheme_id)
        .send()
        .await
        .expect("the attached scheme reads back by id");

    assert_eq!(scheme.id, Some(scheme_id), "the scheme read back is the scheme asked for");
    assert!(!scheme.name.is_empty(), "a scheme carries a name");
    assert!(
        scheme.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a scheme carries its own address: {:?}",
        scheme.self_,
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_schemes_on_the_site_including_that_one() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    let all = match cloud().permission_schemes().get_all_permission_schemes().send().await {
        Ok(all) => all,
        Err(error) => {
            assert!(error.is_forbidden(), "a token that may not list every scheme is refused typed: {error}");

            return;
        }
    };

    let ids: Vec<i64> = all.permission_schemes.unwrap_or_default().into_iter().filter_map(|scheme| scheme.id).collect();

    assert!(ids.contains(&scheme_id), "the site listing carries the scheme the test project is attached to");
}

/// The endpoint documents `expand` as controlling whether grants come back, and then returns them either way.
/// That is the shape callers actually depend on, so it is asserted rather than the documentation.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_every_grant_whether_or_not_expand_asks_for_it() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    let plain =
        cloud().permission_schemes().get_permission_scheme(scheme_id).send().await.expect("the scheme reads back");

    let expanded = cloud()
        .permission_schemes()
        .get_permission_scheme(scheme_id)
        .expand(GetPermissionSchemeRequestExpand::One(GetPermissionSchemeRequestExpandValue::Permissions))
        .send()
        .await
        .expect("the expand parameter is accepted");

    let plain_grants = plain.permissions.unwrap_or_default();
    let expanded_grants = expanded.permissions.unwrap_or_default();

    assert!(!plain_grants.is_empty(), "a scheme in use carries grants");
    assert_eq!(expanded_grants.len(), plain_grants.len(), "expanding for permissions changes nothing about them");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_each_grant_as_a_permission_paired_with_a_holder() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    let grants = cloud()
        .permission_schemes()
        .get_permission_scheme_grants(scheme_id)
        .send()
        .await
        .expect("the grants of the attached scheme read back");

    let grants = grants.permissions.unwrap_or_default();

    assert!(!grants.is_empty(), "a scheme in use carries grants");

    for grant in &grants {
        assert!(grant.id.is_some_and(|id| id > 0), "a grant carries an id");
        assert!(grant.permission.as_ref().is_some_and(|name| !name.is_empty()), "a grant names a permission");
        assert!(grant.holder.as_ref().is_some_and(|holder| !holder.r#type.is_empty()), "a grant names who holds it",);
    }
}

/// Why the write suites can delete the issues they create: a grant of *Delete Issues* to a project role the account
/// sits in. Asserted here so that a 403 in another suite has somewhere to point.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn explains_why_this_suite_can_delete_issues_at_all() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    let grants = cloud()
        .permission_schemes()
        .get_permission_scheme_grants(scheme_id)
        .send()
        .await
        .expect("the grants of the attached scheme read back");

    let delete_grants: Vec<_> = grants
        .permissions
        .unwrap_or_default()
        .into_iter()
        .filter(|grant| grant.permission.as_deref() == Some("DELETE_ISSUES"))
        .collect();

    assert!(!delete_grants.is_empty(), "the attached scheme grants Delete Issues to somebody");
    assert!(
        delete_grants.iter().any(|grant| grant.holder.as_ref().is_some_and(|holder| holder.r#type == "projectRole")),
        "Delete Issues reaches the test account through a project role",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_grant_by_id() {
    let Some(scheme_id) = attached_scheme_id().await else {
        return;
    };

    let grants = cloud()
        .permission_schemes()
        .get_permission_scheme_grants(scheme_id)
        .send()
        .await
        .expect("the grants of the attached scheme read back");

    let sample = grants.permissions.unwrap_or_default().into_iter().next().expect("a scheme in use carries grants");
    let permission_id = sample.id.expect("a grant carries an id");

    let grant = cloud()
        .permission_schemes()
        .get_permission_scheme_grant(scheme_id, permission_id)
        .send()
        .await
        .expect("a grant reads back on its own");

    assert_eq!(grant.id, sample.id, "the grant read back is the grant asked for");
    assert_eq!(grant.permission, sample.permission, "reading a grant alone reports the same permission");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_scheme_as_a_typed_error() {
    let error = cloud()
        .permission_schemes()
        .get_permission_scheme(99_999_999)
        .send()
        .await
        .expect_err("a scheme that does not exist cannot be read");

    assert!(error.is_not_found() || error.is_forbidden(), "{error}");
}

/// The destructive path, proven through its error channel and never aimed at a scheme that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path_without_ever_aiming_it_at_a_real_scheme() {
    let error = cloud()
        .permission_schemes()
        .delete_permission_scheme(99_999_999)
        .send()
        .await
        .expect_err("a scheme that does not exist cannot be deleted");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The id of the permission scheme the test project is attached to, where the token may see it.
///
/// Reading the attachment needs *Administer Jira* or *Administer Projects*. A token with neither must be refused in a
/// way the caller can recognise, so the refusal is asserted here rather than being silently swallowed by the tests
/// that stand down on it.
async fn attached_scheme_id() -> Option<i64> {
    match cloud().project_permission_schemes().get_assigned_permission_scheme(TEST_PROJECT_KEY).send().await {
        Ok(scheme) => Some(scheme.id.expect("an attached permission scheme carries an id")),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.is_not_found() || error.status() == Some(401),
                "a token that may not read the project configuration is refused typed: {error}",
            );

            None
        }
    }
}
