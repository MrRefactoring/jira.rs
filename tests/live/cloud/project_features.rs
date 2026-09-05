//! The per-project feature toggles, read-only.
//!
//! Toggling a feature changes what a project's users see — turning off the backlog or the board removes it from their
//! navigation — and the change is scoped to nothing smaller than the project. The test project is the one every other
//! live suite runs in, so a failed restore would break them all; the toggle is therefore only ever pointed at a
//! feature key that does not exist.
//!
//! The read half is worth having because it explains what several other suites find: whether a project has a board, a
//! backlog or sprints at all is a per-project toggle, not a property of the site.

use jira::cloud::{ProjectFeatureState, ProjectFeatureState2, ProjectFeatureStateState};

use crate::harness::{TEST_PROJECT_KEY, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_features_of_the_test_project_each_with_a_state() {
    let container = cloud()
        .project_features()
        .get_features_for_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project lists its features");

    let features = container.features.expect("the container carries a feature list");

    assert!(!features.is_empty(), "the test project has features");

    for feature in &features {
        assert!(feature.feature.as_deref().is_some_and(|key| !key.is_empty()), "every feature is keyed: {feature:?}");
        assert!(
            feature.state.as_ref().is_some_and(ProjectFeatureState2::is_documented),
            "every feature carries a documented state: {:?}",
            feature.state,
        );
        assert!(feature.prerequisites.is_some(), "every feature lists its prerequisites: {feature:?}");
        assert!(feature.toggle_locked.is_some(), "every feature says whether it can be toggled: {feature:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn records_which_features_cannot_be_toggled_at_all() {
    let features = cloud()
        .project_features()
        .get_features_for_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project lists its features")
        .features
        .unwrap_or_default();

    assert!(!features.is_empty(), "the test project has features");
    assert!(
        features.iter().all(|feature| feature.toggle_locked.is_some()),
        "the lock is recorded for every feature, not only the locked ones",
    );

    for feature in features.iter().filter(|feature| feature.toggle_locked == Some(true)) {
        assert!(feature.feature.as_deref().is_some_and(|key| !key.is_empty()), "a locked feature is still keyed");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn names_prerequisites_as_feature_keys_that_exist_on_the_same_project() {
    let features = cloud()
        .project_features()
        .get_features_for_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project lists its features")
        .features
        .unwrap_or_default();

    let keys: Vec<String> = features.iter().filter_map(|feature| feature.feature.clone()).collect();

    assert!(!keys.is_empty(), "the test project has features");

    for feature in &features {
        for prerequisite in feature.prerequisites.clone().unwrap_or_default() {
            assert!(keys.contains(&prerequisite), "a prerequisite names a feature of the same project: {prerequisite}");
        }
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_the_project_by_id_as_well_as_by_key() {
    let id = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back")
        .id
        .expect("a project carries an id");

    let by_id = cloud()
        .project_features()
        .get_features_for_project(&id)
        .send()
        .await
        .expect("the features read back by project id");

    let by_key = cloud()
        .project_features()
        .get_features_for_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the features read back by project key");

    assert_eq!(
        by_id.features.map(|features| features.len()),
        by_key.features.map(|features| features.len()),
        "the same project answers the same features either way",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_project_as_a_typed_not_found() {
    let error = cloud()
        .project_features()
        .get_features_for_project("NOSUCHPROJECT")
        .send()
        .await
        .expect_err("a project that does not exist has no features");

    assert!(error.is_not_found(), "{error}");
}

/// The toggle is pinned through its error channel only: a real feature key would change what every other suite sees.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_toggle_without_ever_aiming_it_at_a_real_feature() {
    let error = cloud()
        .project_features()
        .toggle_feature_for_project(
            TEST_PROJECT_KEY,
            "no.such.feature.jrs",
            ProjectFeatureState { state: Some(ProjectFeatureStateState::Enabled) },
        )
        .send()
        .await
        .expect_err("a feature that does not exist cannot be toggled");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}
