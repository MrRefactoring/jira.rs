//! Ported from jira.js/tests/live/cloud/projectValidation.test.ts.
//!
//! Both APIs here are pure read helpers with no write half at all — they exist to answer a question a form asks while
//! the user is still typing, which is why they are grouped together.
//!
//! The behaviour worth pinning is that validation does not fail. `validate_project_key` answers 200 with a list of
//! complaints, and `get_valid_project_key` goes further and *invents* a different key rather than refusing — so a
//! caller who ignores the response and uses their own key is not creating the project they think they are.

use jira::cloud::{CustomTemplatesProjectDetails, ErrorCollection, ProjectCustomTemplateCreateRequestDTO};

use crate::harness::{TEST_PROJECT_KEY, cloud};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_validation_complaints_rather_than_failing() {
    let result = cloud()
        .project_key_and_name_validation()
        .validate_project_key()
        .key("lowercase")
        .send()
        .await
        .expect("an invalid key is answered, not refused");

    assert!(
        result.error_messages.is_some() || result.errors.is_some(),
        "the answer carries a complaint channel rather than an error: {result:?}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn accepts_a_key_that_is_genuinely_valid() {
    let result = cloud()
        .project_key_and_name_validation()
        .validate_project_key()
        .key("JRSVALID")
        .send()
        .await
        .expect("a well-formed, unused key is validated");

    assert_eq!(complaints(&result), Vec::<String>::new(), "a key nothing objects to draws no complaints");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_key_already_taken_by_an_existing_project() {
    let result = cloud()
        .project_key_and_name_validation()
        .validate_project_key()
        .key(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("a taken key is answered, not refused");

    assert!(!complaints(&result).is_empty(), "a key another project already holds draws a complaint");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn invents_a_usable_key_rather_than_refusing_an_unusable_one() {
    let suggested = cloud()
        .project_key_and_name_validation()
        .get_valid_project_key()
        .key(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("a taken key is answered with one that is free");

    assert!(!suggested.is_empty(), "the suggestion is a key, not an empty string");
    assert_ne!(suggested, TEST_PROJECT_KEY, "a caller who ignores the answer is not creating the project they think");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn invents_a_usable_name_the_same_way() {
    let name = cloud()
        .projects()
        .get_project(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the test project reads back by key")
        .name
        .expect("a project carries a name");

    let suggested = cloud()
        .project_key_and_name_validation()
        .get_valid_project_name(&name)
        .send()
        .await
        .expect("a taken name is answered with one that is free");

    assert!(!suggested.is_empty(), "the suggestion is a name, not an empty string");
    assert_ne!(suggested, name, "the name a project already holds is not offered back");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn finds_users_and_groups_in_one_call_each_with_a_total() {
    let result = cloud()
        .group_and_user_picker()
        .find_users_and_groups("")
        .max_results(5)
        .send()
        .await
        .expect("the picker answers a broad query");

    assert!(
        result.users.as_ref().and_then(|users| users.total).is_some(),
        "the users half reports a total: {result:?}"
    );
    assert!(
        result.groups.as_ref().and_then(|groups| groups.total).is_some(),
        "the groups half reports a total: {result:?}",
    );
    assert!(
        result.users.as_ref().and_then(|users| users.users.as_ref()).map_or(0, Vec::len) <= 5,
        "the limit caps the users half",
    );
    assert!(
        result.groups.as_ref().and_then(|groups| groups.groups.as_ref()).map_or(0, Vec::len) <= 5,
        "the limit caps the groups half",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_an_unmatched_picker_query_with_empty_halves_rather_than_an_error() {
    let result = cloud()
        .group_and_user_picker()
        .find_users_and_groups("nobodymatchesthisatall")
        .send()
        .await
        .expect("a query nothing matches is still a query");

    assert!(
        result.users.as_ref().and_then(|users| users.users.as_ref()).map_or(0, Vec::len) == 0,
        "nothing matched, so the users half is empty: {result:?}",
    );
    assert!(
        result.groups.as_ref().and_then(|groups| groups.groups.as_ref()).map_or(0, Vec::len) == 0,
        "nothing matched, so the groups half is empty: {result:?}",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_avatars_available_to_a_project() {
    let avatars = cloud()
        .project_avatars()
        .get_all_project_avatars(TEST_PROJECT_KEY)
        .send()
        .await
        .expect("the project lists the avatars available to it");

    assert!(!avatars.system.unwrap_or_default().is_empty(), "a project always has Jira's own avatars to choose from");
}

/// The destructive path, proven through its error channel and aimed at details no site would accept.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_project_creation_from_a_template_without_ever_creating_one() {
    let error = cloud()
        .project_templates()
        .create_project_with_custom_template(ProjectCustomTemplateCreateRequestDTO {
            details: Some(CustomTemplatesProjectDetails {
                key: Some("lowercase".to_owned()),
                name: Some(String::new()),
                lead_account_id: Some(String::new()),
                ..CustomTemplatesProjectDetails::default()
            }),
            ..ProjectCustomTemplateCreateRequestDTO::default()
        })
        .send()
        .await
        .expect_err("a lowercase key and an empty name describe no project Jira would create");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The two channels validation answers on, flattened: free-text messages and complaints keyed by parameter.
fn complaints(result: &ErrorCollection) -> Vec<String> {
    let messages = result.error_messages.clone().unwrap_or_default();
    let by_parameter = result.errors.clone().unwrap_or_default().into_values().map(|value| value.to_string());

    messages.into_iter().chain(by_parameter).collect()
}
