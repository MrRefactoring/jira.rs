//! The project type catalogue, read-only.
//!
//! The interesting part is the pair of "all" and "accessible" variants: they look interchangeable in the types and
//! are not — one lists what Jira defines, the other what this site is licensed for. Asserting that the accessible set
//! is a subset is the only way that distinction stays visible.

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_every_project_type_jira_defines_each_fully_typed() {
    let types = cloud().project_types().get_all_project_types().send().await.expect("the site lists its project types");

    assert!(!types.is_empty(), "Jira defines project types");

    for project_type in &types {
        assert!(
            project_type.key.as_deref().is_some_and(|key| !key.is_empty()),
            "every type is keyed: {project_type:?}",
        );
        assert!(project_type.formatted_key.is_some(), "every type carries a formatted key: {project_type:?}");
        assert!(project_type.description_i18n_key.is_some(), "every type carries a description key: {project_type:?}");
    }

    let keys: Vec<String> = types.iter().filter_map(|project_type| project_type.key.clone()).collect();

    assert!(keys.iter().any(|key| key == "software"), "software is one of them: {keys:?}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_accessible_types_as_a_subset_of_all_types() {
    let all = cloud().project_types().get_all_project_types().send().await.expect("the site lists its project types");
    let accessible = cloud()
        .project_types()
        .get_all_accessible_project_types()
        .send()
        .await
        .expect("the site lists the project types it is licensed for");

    let all_keys: Vec<String> = all.iter().filter_map(|project_type| project_type.key.clone()).collect();

    assert!(!accessible.is_empty(), "the site is licensed for at least one project type");
    assert!(
        accessible.iter().all(|project_type| project_type.key.as_ref().is_some_and(|key| all_keys.contains(key))),
        "every licensed type is one Jira defines: {all_keys:?}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_type_by_key_through_both_variants() {
    let by_key = cloud()
        .project_types()
        .get_project_type_by_key("software")
        .send()
        .await
        .expect("the software project type reads back");

    let accessible = cloud()
        .project_types()
        .get_accessible_project_type_by_key("software")
        .send()
        .await
        .expect("the software project type is accessible to this site");

    assert_eq!(by_key.key.as_deref(), Some("software"));
    assert_eq!(accessible.key.as_deref(), Some("software"));
    assert_eq!(accessible.formatted_key, by_key.formatted_key, "both variants describe the same type");
}

/// The key is an open enum, so a value outside it compiles: the refusal has to come from the site.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_key_outside_the_enum_with_a_typed_error() {
    let error = cloud()
        .project_types()
        .get_project_type_by_key("no_such_type")
        .send()
        .await
        .expect_err("a project type that does not exist cannot be read");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}
