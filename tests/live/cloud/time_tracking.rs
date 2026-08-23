use jira::cloud::{AnnouncementBannerConfigurationUpdate, TimeTrackingProvider};

use crate::harness::cloud;

/// Time tracking and the neighbouring site-wide settings, read-only.
///
/// Every write here is site-wide with no smaller scope: changing the time-tracking provider silently reinterprets
/// every worklog on the tenant, the default navigator columns change what every user sees in search results, and the
/// announcement banner is displayed to everyone on every page. So the writes are pinned only through their error
/// channel, aimed at values that cannot be accepted.
///
/// The configuration read here is not incidental — the worklogs suite logs `1h 30m` and asserts 5400 seconds, and
/// that arithmetic holds only because of the working-hours settings this file reads.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_shared_time_tracking_configuration() {
    let configuration = match cloud().time_tracking().get_shared_time_tracking_configuration().send().await {
        Ok(configuration) => configuration,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    assert!(!configuration.default_unit.as_str().is_empty(), "the configuration names a default unit");
    assert!(!configuration.time_format.as_str().is_empty(), "the configuration names a time format");
    assert!(configuration.working_hours_per_day >= 0.0, "a working day is a real number of hours");
    assert!(configuration.working_days_per_week >= 0.0, "a working week is a real number of days");
}

/// What the worklogs suite's `1h 30m` arithmetic rests on.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn agrees_with_what_the_worklog_suite_depends_on() {
    let configuration = match cloud().time_tracking().get_shared_time_tracking_configuration().send().await {
        Ok(configuration) => configuration,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    assert!(configuration.working_hours_per_day > 0.0, "a working day is longer than nothing");
    assert!(configuration.working_days_per_week > 0.0, "a working week is longer than nothing");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_available_time_tracking_providers() {
    let providers = match cloud().time_tracking().get_available_time_tracking_implementations().send().await {
        Ok(providers) => providers,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    for provider in &providers {
        assert!(!provider.key.is_empty(), "a provider carries a key");
        assert!(provider.name.as_ref().is_some_and(|name| !name.is_empty()), "a provider carries a name");
    }
}

/// The operation is declared to return nothing and returns the selected provider anyway.
///
/// Both halves are asserted: the modelled call succeeds against a body the schema says is empty, and the unmodelled
/// one shows what actually arrived.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_selected_provider_despite_being_declared_to_return_nothing() {
    let body = match cloud().time_tracking().get_selected_time_tracking_implementation().send_raw().await {
        Ok(body) => body,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    assert_eq!(
        body.get("key").and_then(serde_json::Value::as_str),
        Some("JIRA"),
        "the site is on Jira's own time tracking",
    );

    cloud()
        .time_tracking()
        .get_selected_time_tracking_implementation()
        .send()
        .await
        .expect("the modelled call, declared to return nothing, still succeeds");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_default_issue_navigator_columns() {
    let columns = match cloud().issue_navigator_settings().get_issue_navigator_default_columns().send().await {
        Ok(columns) => columns,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    for column in &columns {
        assert!(column.label.as_ref().is_some_and(|label| !label.is_empty()), "a column carries a label");
        assert!(column.value.as_ref().is_some_and(|value| !value.is_empty()), "a column carries a field value");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_the_announcement_banner_configuration() {
    let banner = match cloud().announcement_banner().get_banner().send().await {
        Ok(banner) => banner,
        Err(error) => {
            assert!(
                error.is_forbidden() || error.status() == Some(401),
                "a token without Administer Jira is refused typed: {error}",
            );

            return;
        }
    };

    assert!(banner.is_dismissible.is_some(), "the banner says whether it can be dismissed");
    assert!(banner.is_enabled.is_some(), "the banner says whether it is on");

    if let Some(visibility) = &banner.visibility {
        assert!(!visibility.as_str().is_empty(), "a banner that names its visibility names it with a value");
    }
}

/// The site-wide writes, proven through their error channel and never aimed at a value that would take effect.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_site_wide_writes() {
    let banner = cloud()
        .announcement_banner()
        .set_banner(AnnouncementBannerConfigurationUpdate {
            message: Some(String::new()),
            is_enabled: Some(true),
            is_dismissible: Some(true),
            visibility: Some("nosuchvisibility".to_owned()),
        })
        .send()
        .await
        .expect_err("a visibility that is neither public nor private cannot be set");

    assert!(banner.status().is_some_and(|status| status >= 400), "{banner}");

    let provider = cloud()
        .time_tracking()
        .select_time_tracking_implementation(TimeTrackingProvider {
            key: "no.such.provider.jrs".to_owned(),
            name: None,
            url: None,
        })
        .send()
        .await
        .expect_err("a provider that is not installed cannot be selected");

    assert!(provider.status().is_some_and(|status| status >= 400), "{provider}");
}
