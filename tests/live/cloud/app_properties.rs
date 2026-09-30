use jira::cloud::BulkRedactionRequest;

use crate::harness::cloud;

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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn refuses_the_forge_property_variants_too() {
    let error = cloud()
        .app_properties()
        .get_forge_app_property_keys()
        .send()
        .await
        .expect_err("only a Forge app can read its own property keys");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
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

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_redaction_job_status_as_unreachable_rather_than_empty() {
    let error = cloud()
        .issue_redaction()
        .get_redaction_status("00000000-0000-0000-0000-000000000000")
        .send()
        .await
        .expect_err("a redaction job that was never submitted has no status");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn never_submits_a_redaction_and_fails_typed_on_the_attempt() {
    let error = cloud()
        .issue_redaction()
        .redact(BulkRedactionRequest { redactions: Some(Vec::new()) })
        .send()
        .await
        .expect_err("an empty redaction is still a redaction a user token may not submit");

    assert!(error.status().is_some_and(|status| (400..500).contains(&status)), "{error}");
}
