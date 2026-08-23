use jira::cloud::ApplicationRole;

use crate::harness::cloud;

/// The application roles API, read-only and admin-gated.
///
/// Both operations need site administration. A token without it must be refused *typed* — that is the part worth
/// pinning, because an untyped rejection here is indistinguishable from a network fault to calling code. Where the
/// token does hold admin, the full shape is asserted instead.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_application_roles_for_an_admin_or_fails_forbidden() {
    let Some(roles) = application_roles().await else {
        return;
    };

    assert!(!roles.is_empty(), "a site always carries the roles its products define");

    for role in &roles {
        assert!(role.key.as_ref().is_some_and(|key| !key.is_empty()), "a role carries a key");
        assert!(role.name.as_ref().is_some_and(|name| !name.is_empty()), "a role carries a name");
        assert!(role.number_of_seats.is_some(), "a role carries its seat count");
    }

    assert!(
        roles.iter().any(|role| role.key.as_deref() == Some("jira-software")),
        "the site the suites run against is a Jira Software site",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_role_by_key() {
    let Some(roles) = application_roles().await else {
        return;
    };

    let sample = roles.first().expect("a site carries at least one application role");
    let key = sample.key.clone().expect("a role carries a key");

    let role = cloud().application_roles().get_application_role(&key).send().await.expect("a role reads back by key");

    assert_eq!(role.key, sample.key, "the role resolved by key is the one the listing named");
    assert_eq!(role.name, sample.name);
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_role_key_as_a_typed_error() {
    let error = cloud()
        .application_roles()
        .get_application_role("no-such-role")
        .send()
        .await
        .expect_err("a role that does not exist cannot be resolved");

    assert!(error.is_not_found() || error.is_forbidden(), "an unknown role is refused typed, never untyped: {error}");
}

/// The role listing, or `None` when the token cannot read it.
///
/// The refusal is asserted here rather than being silently swallowed by the tests that stand down on it: a token
/// without *Administer Jira* must be told so in a way the caller can branch on.
async fn application_roles() -> Option<Vec<ApplicationRole>> {
    match cloud().application_roles().get_all_application_roles().send().await {
        Ok(roles) => Some(roles),
        Err(error) => {
            assert!(error.is_forbidden(), "a token without Administer Jira is refused as forbidden: {error}");

            None
        }
    }
}
