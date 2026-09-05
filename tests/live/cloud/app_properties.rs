use jira::cloud::BulkRedactionRequest;

use crate::harness::cloud;

/// The app properties API and the app-only modules next to it: the Forge property variants, the UI modification
/// reads, and issue redaction.
///
/// All app-only, and this file's job is to say so precisely. Unlike the webhook endpoints — which refuse a user token
/// outright — these are addressed by an app key a user token has no claim to, so the failure is about *identity*
/// rather than permission and the status alone does not convey that.
///
/// The distinction matters because these look like the entity-property endpoints covered elsewhere (issues, projects,
/// users, issue types), share their shape, and are the one member of that family a user token cannot use.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_to_list_app_properties_for_user_credentials() {
    let error = cloud()
        .app_properties()
        .get_addon_properties("com.example.no.such.app")
        .send()
        .await
        .expect_err("a user token has no app to read properties for");

    let status = error.status().expect("the refusal comes from the site rather than from the transport");

    assert!((400..500).contains(&status), "the refusal is about the caller, not the server: {error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_a_single_app_property_the_same_way() {
    let error = cloud()
        .app_properties()
        .get_addon_property("com.example.no.such.app", "jirars.livetest")
        .send()
        .await
        .expect_err("one property is no more reachable than the listing");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The write is aimed at an app key that does not exist, so no app's state is ever touched — the refusal is the
/// whole assertion.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_write_so_no_app_state_is_ever_touched() {
    let error = cloud()
        .app_properties()
        .put_addon_property(
            "com.example.no.such.app",
            "jirars.livetest",
            [("written".to_owned(), serde_json::json!(false))].into_iter().collect(),
        )
        .send()
        .await
        .expect_err("a user token cannot write an app's property");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The Forge variants are addressed by the calling app rather than by a key in the path, so they carry no argument to
/// get wrong — and are refused all the same.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_forge_property_variants_too() {
    let error = cloud()
        .app_properties()
        .get_forge_app_property_keys()
        .send()
        .await
        .expect_err("only a Forge app can read its own property keys");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_ui_modification_reads_which_are_app_scoped_as_well() {
    let error = cloud()
        .ui_modifications_apps()
        .get_ui_modifications()
        .max_results(5)
        .send()
        .await
        .expect_err("UI modifications belong to the app that declared them");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// A redaction job id that cannot exist, so the answer separates "no such job" from "no permission to ask".
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_redaction_job_status_as_unreachable_rather_than_empty() {
    let error = cloud()
        .issue_redaction()
        .get_redaction_status("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .expect_err("a redaction job that was never submitted has no status");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// Redaction is irreversible, so the request carries nothing to redact and is proven only through its error channel.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn never_submits_a_redaction_and_fails_typed_on_the_attempt() {
    let error = cloud()
        .issue_redaction()
        .redact(BulkRedactionRequest { redactions: Some(Vec::new()) })
        .send()
        .await
        .expect_err("an empty redaction is still a redaction a user token may not submit");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}
