//! The project role catalogue, read-only.
//!
//! Roles are the layer between a permission scheme and a person: the scheme grants a permission to a role, and role
//! membership is per-project. That indirection is what makes this suite worth having — the live credentials reach
//! this project's issues *because* the account sits in a role here, and these tests make that chain visible rather
//! than folklore.
//!
//! Creating or deleting a role is site-wide configuration, so neither is exercised.

use std::collections::HashMap;

use crate::harness::{TEST_PROJECT_KEY, cloud};

/// The roles of the test project, as name-to-URL pairs.
async fn project_roles() -> HashMap<String, String> {
    cloud().project_roles().get_project_roles(TEST_PROJECT_KEY).send().await.expect("the test project lists its roles")
}

/// The id of the `Administrators` role, which the listing only carries as the last segment of the role's URL.
fn administrators_id(roles: &HashMap<String, String>) -> i64 {
    roles
        .get("Administrators")
        .and_then(|url| url.rsplit('/').next())
        .and_then(|id| id.parse().ok())
        .expect("the Administrators role url ends in its numeric id")
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_roles_available_in_the_test_project_as_name_to_url_pairs() {
    let roles = project_roles().await;

    assert!(!roles.is_empty(), "the test project has roles");
    assert!(roles.contains_key("Administrators"), "the test project has an Administrators role: {roles:?}");

    for (name, url) in &roles {
        let id = url.rsplit('/').next().unwrap_or_default();

        assert!(url.starts_with("https://"), "the {name} role is addressed absolutely: {url}");
        assert!(url.contains("/role/"), "the {name} role is addressed as a role: {url}");
        assert!(!id.is_empty() && id.chars().all(|c| c.is_ascii_digit()), "the {name} role url ends in an id: {url}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_role_by_the_id_embedded_in_that_url() {
    let id = administrators_id(&project_roles().await);
    let role = cloud()
        .project_roles()
        .get_project_role(TEST_PROJECT_KEY, id)
        .send()
        .await
        .expect("the role reads back by the id its url carries");

    assert_eq!(role.name.as_deref(), Some("Administrators"));
    assert_eq!(role.id, Some(id));
    assert!(role.actors.is_some(), "a role read in a project context carries its membership");
}

/// The permission chain the whole live suite depends on, stated once.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn shows_the_account_holding_the_role_that_grants_it_access_here() {
    let account_id = cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site knows the caller")
        .account_id
        .expect("the caller carries an account id");

    let id = administrators_id(&project_roles().await);
    let role =
        cloud().project_roles().get_project_role(TEST_PROJECT_KEY, id).send().await.expect("the role reads back");
    let actors = role.actors.unwrap_or_default();

    assert!(!actors.is_empty(), "the role that grants this suite its access has members");
    assert!(
        actors
            .iter()
            .any(|actor| actor.actor_user.as_ref().and_then(|user| user.account_id.as_deref())
                == Some(account_id.as_str())),
        "the test account is one of them",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_every_role_with_its_actors_in_one_call() {
    let details = cloud()
        .project_roles()
        .get_project_role_details(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project describes its roles");

    assert!(!details.is_empty(), "the test project has roles to describe");

    for role in &details {
        assert!(role.name.as_deref().is_some_and(|name| !name.is_empty()), "every role is named: {role:?}");
        assert!(role.id.is_some(), "every role carries an id: {:?}", role.name);
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn narrows_the_details_to_roles_the_caller_is_actually_in() {
    let all = cloud()
        .project_roles()
        .get_project_role_details(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project describes its roles");

    let mine = cloud()
        .project_roles()
        .get_project_role_details(TEST_PROJECT_KEY)
        .current_member(true)
        .send()
        .await
        .expect("the currentMember parameter is accepted");

    let names: Vec<String> = mine.iter().filter_map(|role| role.name.clone()).collect();

    assert!(!mine.is_empty(), "the caller is in at least one role here");
    assert!(mine.len() <= all.len(), "the caller's roles are a subset of the project's roles");
    assert!(names.iter().any(|name| name == "Administrators"), "Administrators is one of them: {names:?}");
}

/// The site-wide role catalogue needs *Administer Jira*, which a project admin does not have.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_site_role_catalogue_for_an_admin_or_fails_typed() {
    match cloud().project_roles().get_all_project_roles().send().await {
        Ok(roles) => {
            let names: Vec<String> = roles.iter().filter_map(|role| role.name.clone()).collect();

            assert!(names.iter().any(|name| name == "Administrators"), "the catalogue holds Administrators: {names:?}");
        }
        Err(error) => assert!(error.is_forbidden(), "a non-admin is refused, not failed: {error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_role_id_as_a_typed_error() {
    let error = cloud()
        .project_roles()
        .get_project_role(TEST_PROJECT_KEY, 99_999_999)
        .send()
        .await
        .expect_err("a role that does not exist cannot be read");

    assert!(error.is_not_found() || error.status() == Some(400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_project_as_a_typed_not_found() {
    let error = cloud()
        .project_roles()
        .get_project_roles("NOSUCHPROJECT")
        .send()
        .await
        .expect_err("a project that does not exist has no roles");

    assert!(error.is_not_found(), "{error}");
}
