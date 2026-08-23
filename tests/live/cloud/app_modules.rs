use jira::cloud::{CustomFieldValueUpdateDetails, WorkflowRulesSearch};

use crate::harness::cloud;

/// The remaining app-only platform modules: the app-owned custom field options, values and configuration, the custom
/// field associations, the JQL function precomputations, the app data policies and the two migration surfaces.
///
/// Grouped because they share one story, and eight files asserting that a 4xx arrives would say less than one file
/// explaining why. All of them act on behalf of an installed app — its custom field types, its JQL functions, its
/// migration state — and a user token has no app to act for.
///
/// Two of them are worth separating out. `issue_custom_field_options_apps` is the app-owned twin of
/// `issue_custom_field_options`, which the context suite covers and which *is* reachable — the two look
/// interchangeable on the client and are not. And `app_data_policies` reports whether a site restricts what apps may
/// read, which is a governance answer a caller may legitimately need without being an app.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_app_owned_custom_field_option_reads() {
    let error = cloud()
        .issue_custom_field_options_apps()
        .get_all_issue_field_options("com.example.no.such.app__field")
        .max_results(5)
        .send()
        .await
        .expect_err("only the app that declared the field type may read its options");

    let status = error.status().expect("the refusal comes from the site rather than from the transport");

    assert!((400..500).contains(&status), "the refusal is about the caller, not the server: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_app_owned_custom_field_value_writes() {
    let error = cloud()
        .issue_custom_field_values_apps()
        .update_custom_field_value(
            "com.example.no.such.app__field",
            CustomFieldValueUpdateDetails { updates: Some(Vec::new()) },
        )
        .send()
        .await
        .expect_err("only the app that owns the field may write its values");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_app_custom_field_configuration_reads() {
    let error = cloud()
        .issue_custom_field_configuration_apps()
        .get_custom_field_configuration("com.example.no.such.app__field")
        .send()
        .await
        .expect_err("a field type no app declared has no configuration to read");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_jql_function_precomputation_reads() {
    let error = cloud()
        .jql_functions_apps()
        .get_precomputations()
        .max_results(5)
        .send()
        .await
        .expect_err("precomputations belong to the app whose JQL function produced them");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// Both migration surfaces act on an app installation that is being moved, so neither has anything to say to a user
/// token — and neither may be aimed at a real transfer, which is why the ids are nil.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_migration_endpoints_which_act_on_an_app_installation() {
    let migration = cloud()
        .app_migration()
        .workflow_rule_search(
            "00000000-0000-0000-0000-000000000000",
            WorkflowRulesSearch {
                expand: None,
                rule_ids: Vec::new(),
                workflow_entity_id: "00000000-0000-0000-0000-000000000000".to_owned(),
            },
        )
        .send()
        .await
        .expect_err("a transfer id no app owns cannot be searched");

    let forge = cloud()
        .migration_of_connect_modules_to_forge()
        .fetch_migration_task("com.example.absent", "absent-field")
        .send()
        .await
        .expect_err("an app that was never installed has no migration task");

    assert!(migration.status().is_some_and(|status| status >= 400), "{migration}");
    assert!(forge.status().is_some_and(|status| status >= 400), "{forge}");
}

/// The one read in the family that is not app-gated: what an app *would* be allowed to see is a property of the site,
/// and a site may answer it or refuse it outright.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_site_data_policy_which_is_not_app_gated() {
    match cloud().app_data_policies().get_policy().send().await {
        Ok(policy) => assert!(
            policy.any_content_blocked.is_some(),
            "a site that answers at all says whether any of its content is blocked",
        ),
        Err(error) => assert!(error.status().is_some_and(|status| status >= 400), "{error}"),
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_per_project_data_policies_alongside_the_site_one() {
    let policies = match cloud().app_data_policies().get_policies().send().await {
        Ok(policies) => policies,
        Err(error) => {
            assert!(error.status().is_some_and(|status| status >= 400), "a refused policy listing is typed: {error}");

            return;
        }
    };

    assert!(policies.project_data_policies.is_some(), "the listing carries a list of projects, empty or not");

    for entry in policies.project_data_policies.iter().flatten() {
        assert!(entry.id.is_some_and(|id| id > 0), "a policy entry names the project it is about");
        assert!(
            entry.data_policy.as_ref().is_some_and(|policy| policy.any_content_blocked.is_some()),
            "a policy entry says whether the project blocks any content",
        );
    }
}

/// The part the library owns across the whole family: whatever the site answers, the failure arrives as a typed
/// status rather than as a transport error or a hang.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_across_the_family() {
    let error = cloud()
        .jql_functions_apps()
        .get_precomputations()
        .send()
        .await
        .expect_err("precomputations are app-only with or without a page size");

    assert!(error.status().is_some(), "the refusal carries a status: {error}");
}
