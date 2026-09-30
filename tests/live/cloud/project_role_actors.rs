use std::collections::HashMap;

use jira::cloud::ActorsMap;

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, await_readable, cloud, is_not_entitled};

async fn project_roles() -> HashMap<String, String> {
    cloud().project_roles().get_project_roles(TEST_PROJECT_KEY).send().await.expect("the test project lists its roles")
}

fn administrators_id(roles: &HashMap<String, String>) -> i64 {
    roles
        .get("Administrators")
        .and_then(|url| url.rsplit('/').next())
        .and_then(|id| id.parse().ok())
        .expect("the Administrators role url ends in its numeric id")
}

async fn current_account_id() -> String {
    cloud()
        .myself()
        .get_current_user()
        .send()
        .await
        .expect("the site knows the caller")
        .account_id
        .expect("the caller carries an account id")
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_actors_of_the_role_that_grants_this_suite_its_access() {
    let id = administrators_id(&project_roles().await);
    let role =
        cloud().project_roles().get_project_role(TEST_PROJECT_KEY, id).send().await.expect("the role reads back");
    let actors = role.actors.expect("a role read in a project context carries its membership");

    assert!(!actors.is_empty(), "the Administrators role has members");

    for actor in &actors {
        assert!(actor.id.is_some(), "every actor carries an id: {actor:?}");
        assert!(actor.r#type.is_some(), "every actor carries a type: {actor:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_the_test_account_among_them_which_is_why_teardown_works() {
    let account_id = current_account_id().await;
    let id = administrators_id(&project_roles().await);
    let role =
        cloud().project_roles().get_project_role(TEST_PROJECT_KEY, id).send().await.expect("the role reads back");
    let actors = role.actors.unwrap_or_default();

    assert!(
        actors
            .iter()
            .any(|actor| actor.actor_user.as_ref().and_then(|user| user.account_id.as_deref())
                == Some(account_id.as_str())),
        "the test account holds the role that lets every other suite clean up after itself",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_default_actors_a_role_gives_new_projects() {
    let id = administrators_id(&project_roles().await);

    match cloud().project_role_actors().get_project_role_actors_for_role(id).send().await {
        Ok(role) => {
            assert!(role.id.is_none(), "the default actors are answered without a role id: {:?}", role.id);

            for actor in role.actors.unwrap_or_default() {
                assert!(actor.id.is_some(), "every default actor carries an id: {actor:?}");
            }
        }
        Err(error) => assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_role_as_a_typed_error() {
    let error = cloud()
        .project_role_actors()
        .get_project_role_actors_for_role(99_999_999)
        .send()
        .await
        .expect_err("a role that does not exist has no default actors");

    assert!(error.is_not_found() || error.status() == Some(400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_an_actor_addition_naming_nobody() {
    let id = administrators_id(&project_roles().await);
    let error = cloud()
        .project_role_actors()
        .add_actor_users(TEST_PROJECT_KEY, id, ActorsMap::default())
        .send()
        .await
        .expect_err("an actor addition naming nobody is refused");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");

    if is_not_entitled(&error) {
        return;
    }

    assert!(!error.is_not_found(), "an empty payload is a bad request, not a missing role: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn silently_succeeds_when_removing_an_actor_that_is_not_in_the_role() {
    let mut tracker = ResourceTracker::new();
    let account_id = current_account_id().await;
    let id = administrators_id(&project_roles().await);

    let restored = account_id.clone();

    tracker.defer(move || {
        let account_id = restored.clone();

        async move {
            let role = cloud().project_roles().get_project_role(TEST_PROJECT_KEY, id).send().await?;
            let held = role.actors.unwrap_or_default().iter().any(|actor| {
                actor.actor_user.as_ref().and_then(|user| user.account_id.as_deref()) == Some(account_id.as_str())
            });

            if held {
                return Ok(());
            }

            cloud()
                .project_role_actors()
                .add_actor_users(
                    TEST_PROJECT_KEY,
                    id,
                    ActorsMap { user: Some(vec![account_id]), ..ActorsMap::default() },
                )
                .send()
                .await
                .map(|_| ())
        }
    });

    let outcome =
        cloud().project_role_actors().delete_actor(TEST_PROJECT_KEY, id).user("no-such-account-id").send().await;

    match outcome {
        Ok(()) => {
            let role = await_readable("the role reads back", || {
                cloud().project_roles().get_project_role(TEST_PROJECT_KEY, id).send()
            })
            .await;

            assert!(
                role.actors.unwrap_or_default().iter().any(|actor| actor
                    .actor_user
                    .as_ref()
                    .and_then(|user| user.account_id.as_deref())
                    == Some(account_id.as_str())),
                "removing a stranger left the test account in the role",
            );
        }
        Err(error) => {
            assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
            assert!(is_not_entitled(&error), "only a plan refusal explains a failure here: {error}");
        }
    }

    tracker.cleanup().await;
}
