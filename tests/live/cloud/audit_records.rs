use crate::harness::{cloud, is_not_entitled, rendered_option};

/// The audit records API, administrator-gated.
///
/// One endpoint, and read-only by nature — there is no way to write an audit record through the API, which is rather
/// the point of an audit log.
///
/// Two things make it worth its own file. Its pagination is unlike anything else in the API: `offset` and `limit`
/// rather than `startAt` and `maxResults`, so paging code copied from a neighbouring endpoint silently reads page one
/// forever. And its date filters are strings the parameter type does nothing to constrain.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_audit_records_for_an_administrator_each_fully_typed() {
    if !may_read_audit_records().await {
        return;
    }

    let page =
        cloud().audit_records().get_audit_records().limit(5).send().await.expect("an administrator reads the log");

    assert!(page.records.is_some(), "the log carries a list of records, empty or not");
    assert!(page.total.is_some(), "the log counts what it paged over");

    for record in page.records.iter().flatten() {
        assert!(record.id.is_some_and(|id| id > 0), "a record carries an id");
        assert!(record.summary.as_ref().is_some_and(|summary| !summary.is_empty()), "a record carries a summary");
        assert!(record.category.as_ref().is_some_and(|category| !category.is_empty()), "a record carries a category");

        let created = rendered_option(&record.created).expect("a record carries the moment it was written");

        assert!(created.contains('T'), "a creation timestamp is ISO 8601: {created}");
    }
}

/// Two different refusals wear the same 403 and mean opposite things: a site whose plan carries no audit log at all,
/// and an administrator-only endpoint reached without administrator rights. Neither is drift, and the assertion is
/// that whichever arrives is typed rather than that one of them does.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_rather_than_silently_empty_without_admin_rights() {
    match cloud().audit_records().get_audit_records().limit(1).send().await {
        Ok(page) => assert_eq!(page.limit, Some(1), "an administrator on an entitled site is answered, not refused"),
        Err(error) => assert!(
            is_not_entitled(&error) || error.is_forbidden() || error.status() == Some(401),
            "a plan without an audit log, or a token without Administer Jira, is refused typed: {error}",
        ),
    }
}

/// The pagination trap: `offset` and `limit`, not `startAt` and `maxResults`. A second page is asked for only where
/// there is one, and it is proven to be a *different* page rather than the first one again.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_with_offset_and_limit_not_start_at_and_max_results() {
    if !may_read_audit_records().await {
        return;
    }

    let first =
        cloud().audit_records().get_audit_records().limit(1).send().await.expect("an administrator reads the log");

    assert_eq!(first.limit, Some(1), "the page size asked for is the page size returned");
    assert_eq!(first.offset, Some(0), "an unoffset request starts at the beginning");
    assert!(first.records.as_ref().map_or(0, Vec::len) <= 1, "a page holds no more than it says it does");

    if first.total.unwrap_or(0) <= 1 {
        return;
    }

    let second = cloud()
        .audit_records()
        .get_audit_records()
        .limit(1)
        .offset(1)
        .send()
        .await
        .expect("the second page is readable");

    assert_eq!(second.offset, Some(1), "the offset asked for is the offset returned");
    assert_ne!(
        second.records.iter().flatten().next().and_then(|record| record.id),
        first.records.iter().flatten().next().and_then(|record| record.id),
        "the offset moves the window rather than being ignored",
    );
}

/// The date filter, anchored on a timestamp the log itself produced rather than on one built here — the parameter is
/// an unconstrained string, and the format Jira accepts is exactly the format it emits.
///
/// Timestamps from one site carry the same UTC offset, so lexicographic order is chronological order.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn narrows_the_window_with_from() {
    if !may_read_audit_records().await {
        return;
    }

    let page =
        cloud().audit_records().get_audit_records().limit(10).send().await.expect("an administrator reads the log");

    let Some(newest) = page.records.iter().flatten().next().and_then(|record| rendered_option(&record.created)) else {
        assert_eq!(page.total, Some(0), "a log with no records says so rather than hiding them");

        return;
    };

    let narrowed = cloud()
        .audit_records()
        .get_audit_records()
        .from(newest.clone())
        .limit(10)
        .send()
        .await
        .expect("a timestamp the log emitted is a timestamp the log accepts");

    for record in narrowed.records.iter().flatten() {
        let created = rendered_option(&record.created).expect("a record carries the moment it was written");

        assert!(created >= newest, "the window excludes what predates it: {created} is before {newest}");
    }
}

/// A filter that matches nothing empties the page without emptying the count — `total` reports the size of the log
/// rather than the size of the match, which is the one thing about this endpoint that reads as a bug and is not.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn answers_a_filter_that_matches_nothing_with_an_empty_page() {
    if !may_read_audit_records().await {
        return;
    }

    let page = cloud()
        .audit_records()
        .get_audit_records()
        .filter("nothingmatchesthisatall")
        .limit(10)
        .send()
        .await
        .expect("the filter is accepted");

    assert_eq!(page.records.as_ref().map_or(0, Vec::len), 0, "nothing matches, so nothing is returned");
    assert!(page.total.is_some_and(|total| total > 0), "the total counts the log, not the match: {:?}", page.total);

    let unfiltered = cloud().audit_records().get_audit_records().limit(1).send().await.expect("the log reads back");

    assert_eq!(page.total, unfiltered.total, "the filter leaves the total untouched");
}

/// Whether the site has an audit log this token may read.
///
/// The log is a paid-plan feature and the endpoint needs *Administer Jira*, so there are two ways to be turned away
/// and both must be recognisable to a caller. The refusal is asserted here rather than being silently swallowed by
/// the tests that stand down on it.
async fn may_read_audit_records() -> bool {
    match cloud().audit_records().get_audit_records().limit(1).send().await {
        Ok(_) => true,
        Err(error) => {
            assert!(
                is_not_entitled(&error) || error.is_forbidden() || error.status() == Some(401),
                "a plan without an audit log, or a token without Administer Jira, is refused typed: {error}",
            );

            false
        }
    }
}
