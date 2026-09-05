use jira::cloud::{ApplicationProperty, SimpleApplicationProperty};

use crate::harness::cloud;

/// The Jira settings API, read-only and firmly so.
///
/// These are site-wide switches: `set_application_property` changes behaviour for every user on the tenant, with no
/// scope smaller than the whole site to contain a mistake. A test that flipped one and failed before restoring it
/// would leave the site altered — so the write is pinned only through its error channel, aimed at a key that cannot
/// exist.
///
/// `get_configuration` is the useful half: it reports which optional features are switched on, and several other
/// suites only make sense in light of it — time tracking, sub-tasks and issue linking are all optional, and code that
/// assumes them works on one tenant and breaks on the next.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_which_optional_features_the_site_has_switched_on() {
    let configuration =
        cloud().jira_settings().get_configuration().send().await.expect("the site reports its settings");

    assert!(configuration.voting_enabled.is_some(), "the site says whether voting is on");
    assert!(configuration.watching_enabled.is_some(), "the site says whether watching is on");
    assert!(configuration.sub_tasks_enabled.is_some(), "the site says whether sub-tasks are on");
    assert!(configuration.attachments_enabled.is_some(), "the site says whether attachments are on");
    assert!(configuration.issue_linking_enabled.is_some(), "the site says whether issue linking is on");
    assert!(configuration.time_tracking_enabled.is_some(), "the site says whether time tracking is on");
}

/// The other suites vote, watch, attach, link and log time. This asserts the site agrees those were allowed, so a
/// switched-off feature surfaces here rather than as an unexplained failure three suites away.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn agrees_with_what_the_other_live_suites_were_able_to_do() {
    let configuration =
        cloud().jira_settings().get_configuration().send().await.expect("the site reports its settings");

    assert_eq!(configuration.voting_enabled, Some(true), "the issue votes suite needs voting");
    assert_eq!(configuration.watching_enabled, Some(true), "the issue watchers suite needs watching");
    assert_eq!(configuration.attachments_enabled, Some(true), "the attachments suite needs attachments");
    assert_eq!(configuration.issue_linking_enabled, Some(true), "the issue links suite needs linking");
    assert_eq!(configuration.time_tracking_enabled, Some(true), "the worklogs suite needs time tracking");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_the_time_tracking_configuration_only_when_it_is_on() {
    let configuration =
        cloud().jira_settings().get_configuration().send().await.expect("the site reports its settings");

    let Some(true) = configuration.time_tracking_enabled else {
        assert!(
            configuration.time_tracking_configuration.is_none(),
            "a site with time tracking off reports no time-tracking configuration",
        );

        return;
    };

    let time_tracking = configuration.time_tracking_configuration.expect("time tracking that is on describes itself");

    assert!(time_tracking.working_hours_per_day > 0.0, "a working day is longer than nothing");
    assert!(time_tracking.working_days_per_week > 0.0, "a working week is longer than nothing");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_advanced_settings_for_an_admin_or_fails_typed() {
    let Some(settings) = advanced_settings().await else {
        return;
    };

    assert!(!settings.is_empty(), "the advanced settings page is never empty on a real site");

    for setting in &settings {
        assert!(setting.id.as_ref().is_some_and(|id| !id.is_empty()), "an advanced setting carries an id");
        assert!(setting.key.as_ref().is_some_and(|key| !key.is_empty()), "an advanced setting carries a key");
        assert!(setting.r#type.as_ref().is_some_and(|kind| !kind.is_empty()), "an advanced setting names its type");
    }
}

/// The `key` parameter changes the response from an array of properties to a single property object, while the
/// declared return type stays an array. There is no tolerant mode to fall back on here: the modelled call fails, and
/// it fails as a schema mismatch that names the endpoint and the shape it got — which is the whole point of the type.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn changes_response_shape_with_the_parameter_which_the_declared_type_does_not_survive() {
    let Some(settings) = advanced_settings().await else {
        return;
    };

    let sample = settings.first().expect("the advanced settings page is never empty on a real site");
    let key = sample.key.clone().expect("an advanced setting carries a key");

    let error = cloud()
        .jira_settings()
        .get_application_property()
        .key(&key)
        .send()
        .await
        .expect_err("the keyed variant answers with an object where the declared type is an array");

    assert!(error.is_schema_mismatch(), "the mismatch is typed as one, not as a serialization accident: {error}");

    let report = error.schema_report().expect("a schema mismatch carries its report");

    assert_eq!(report.endpoint, "GET /rest/api/3/application-properties", "the report names the endpoint");

    let mismatch = report.issues.first().expect("a report names at least one place the shapes disagreed");

    assert_eq!(mismatch.path, "", "the disagreement is at the response root");
    assert_eq!(mismatch.received, "object", "an object arrived");
    assert!(mismatch.expected.contains("sequence"), "an array was expected, got {}", mismatch.expected);

    let raw = cloud()
        .jira_settings()
        .get_application_property()
        .key(&key)
        .send_raw()
        .await
        .expect("the same request reads fine unmodelled");

    assert_eq!(
        raw.get("key").and_then(serde_json::Value::as_str),
        Some(key.as_str()),
        "unmodelled, the keyed variant is one property rather than a list",
    );

    let filtered = cloud()
        .jira_settings()
        .get_application_property()
        .key_filter(&key)
        .send()
        .await
        .expect("the filtered variant stays the array the type declares");

    assert!(
        filtered.iter().any(|property| property.key.as_deref() == Some(key.as_str())),
        "the filter keeps the property it was pointed at",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_property_key_as_a_typed_error() {
    let error = cloud()
        .jira_settings()
        .get_application_property()
        .key("no.such.property.jrs")
        .send()
        .await
        .expect_err("a property that does not exist cannot be read");

    assert!(
        error.is_not_found() || error.is_forbidden() || error.status() == Some(400),
        "an unknown key is refused typed: {error}",
    );
}

/// The site-wide write, proven through its error channel and never aimed at a setting that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_site_wide_write() {
    let error = cloud()
        .jira_settings()
        .set_application_property(
            "no.such.property.jrs",
            SimpleApplicationProperty { value: Some("x".to_owned()), ..SimpleApplicationProperty::default() },
        )
        .send()
        .await
        .expect_err("a property that does not exist cannot be set");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");
}

/// The advanced settings, or `None` when the token cannot read them.
///
/// Every read on this page needs *Administer Jira*. The refusal is asserted here rather than being silently swallowed
/// by the tests that stand down on it.
async fn advanced_settings() -> Option<Vec<ApplicationProperty>> {
    match cloud().jira_settings().get_advanced_settings().send().await {
        Ok(settings) => Some(settings),
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            None
        }
    }
}
