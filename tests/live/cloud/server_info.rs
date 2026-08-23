//! The smallest endpoint on the site, and for that reason the most useful one to pin precisely: it is the first call
//! most callers make, and the cheapest place to catch a broken base URL, a broken auth header, or a schema that has
//! drifted from what Cloud actually sends.

use crate::harness::{cloud, require_live_env};

/// Whether the value is a timestamp with real field values rather than merely a string of the right shape.
fn parses_as_a_date(value: &str) -> bool {
    let Some((date, time)) = value.split_once('T') else {
        return false;
    };
    let mut fields = date.split('-');
    let Some(Ok(year)) = fields.next().map(str::parse::<u32>) else {
        return false;
    };
    let Some(Ok(month)) = fields.next().map(str::parse::<u32>) else {
        return false;
    };
    let Some(Ok(day)) = fields.next().map(str::parse::<u32>) else {
        return false;
    };

    (2000..2100).contains(&year) && (1..=12).contains(&month) && (1..=31).contains(&day) && time.contains(':')
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_site_identity_typed_as_server_information_declares() {
    let info = cloud().server_info().get_server_info().send().await.expect("the site names itself");

    assert!(info.base_url.as_deref().is_some_and(|url| url.starts_with("https://")), "{:?}", info.base_url);
    assert_eq!(info.deployment_type.as_deref(), Some("Cloud"), "these suites run against Cloud, not Data Center");
    assert!(info.version.as_deref().is_some_and(|version| !version.is_empty()), "a site reports its version");
    assert!(!info.version_numbers.unwrap_or_default().is_empty(), "the version is also given field by field");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_a_build_date_and_server_time_that_parse_as_real_dates() {
    let info = cloud().server_info().get_server_info().send().await.expect("the site names itself");

    let build_date = info.build_date.expect("a site reports when it was built");
    let server_time = info.server_time.expect("a site reports its own clock");

    assert!(parses_as_a_date(&build_date), "the build date is a date: {build_date}");
    assert!(parses_as_a_date(&server_time), "the server time is a date: {server_time}");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn agrees_with_the_host_the_client_was_configured_with() {
    let info = cloud().server_info().get_server_info().send().await.expect("the site names itself");
    let configured = require_live_env().host;

    assert_eq!(
        info.base_url.as_deref().map(|url| url.trim_end_matches('/')),
        Some(configured.as_str()),
        "the site answering is the site the client was pointed at",
    );
}
