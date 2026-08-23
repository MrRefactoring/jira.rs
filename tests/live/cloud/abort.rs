//! Ported from jira.js/tests/live/cloud/abort.test.ts.
//!
//! Cancellation, against a real request rather than a stubbed one.
//!
//! The signal has not gone missing — it was never the library's to carry. Where the TypeScript client takes an
//! `AbortSignal` per request, a Rust request is cancelled by dropping the future that drives it, and a deadline is
//! [`tokio::time::timeout`] wrapped around that future. Cancellation is the runtime's, so the client needs no
//! parameter for it and this suite passes none.
//!
//! What is worth proving survives the translation intact: a cancelled request stops instead of running to
//! completion, the cancellation arrives as the runtime's own error rather than dressed up as a `jira::Error`, and
//! the shared client the cancelled request was made through is still usable for the next one.

use std::time::{Duration, Instant};

use crate::harness::cloud;

/// Shorter than any round trip to Atlassian, so the deadline always wins the race.
const IMPOSSIBLE: Duration = Duration::from_millis(1);

/// Long enough for a page of projects on a slow day, short enough that a hang fails the run rather than stalling it.
const GENEROUS: Duration = Duration::from_secs(30);

/// How long a cancellation is allowed to take before it stops counting as prompt.
const PROMPTLY: Duration = Duration::from_secs(2);

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn cancels_an_in_flight_request_and_leaves_the_client_usable() {
    let started = Instant::now();

    let cancelled = tokio::time::timeout(IMPOSSIBLE, cloud().projects().search_projects().max_results(50).send())
        .await
        .expect_err("a millisecond is not enough for a round trip to Jira");

    // The counterpart of the abort reason being rethrown unwrapped: the deadline belongs to the runtime, so what
    // comes back is `Elapsed` and not a `jira::Error` claiming the network failed.
    assert_eq!(cancelled.to_string(), "deadline has elapsed", "the runtime's own error, not the library's");
    assert!(started.elapsed() < PROMPTLY, "the request stops at the deadline, took {:?}", started.elapsed());

    // Dropping a request mid-flight must not leave the connection pool or the auth state in a condition the next
    // call trips over — one client is shared by every suite in the run.
    let page = tokio::time::timeout(GENEROUS, cloud().projects().search_projects().max_results(1).send())
        .await
        .expect("the next request is not held up by the cancelled one")
        .expect("the shared client still works after a cancellation");

    assert!(!page.values.is_empty(), "the site lists at least one project");
}

/// A deadline that has already passed is the equivalent of a signal that is aborted before the request is made.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_promptly_when_the_deadline_has_already_passed() {
    let started = Instant::now();

    let expired = tokio::time::timeout(Duration::ZERO, cloud().projects().search_projects().max_results(1).send())
        .await
        .expect_err("a deadline of zero is already past when the request starts");

    // Nothing here can be mistaken for a network or an auth failure: no `jira::Error` was ever produced, because no
    // response was ever read. That the TypeScript suite has to assert it is a consequence of one error type for
    // everything.
    assert_eq!(expired.to_string(), "deadline has elapsed");
    assert!(started.elapsed() < PROMPTLY, "an expired deadline fails at once, took {:?}", started.elapsed());
}

/// The other way a request is cancelled: the task driving it is taken away underneath it.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn abandons_the_request_when_the_task_driving_it_is_dropped() {
    let pending = tokio::spawn(async { cloud().projects().search_projects().max_results(50).send().await });

    // Long enough for the request to reach the socket, far too short for an answer to come back from it.
    tokio::time::sleep(Duration::from_millis(5)).await;
    pending.abort();

    let outcome = pending.await.expect_err("an aborted task hands back no result");

    assert!(outcome.is_cancelled(), "the request stopped rather than completing: {outcome}");

    let page = cloud()
        .projects()
        .search_projects()
        .max_results(1)
        .send()
        .await
        .expect("an abandoned request does not poison the client that made it");

    assert!(!page.values.is_empty(), "the site lists at least one project");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lets_a_request_finish_when_the_deadline_is_generous() {
    let page = tokio::time::timeout(GENEROUS, cloud().projects().search_projects().max_results(1).send())
        .await
        .expect("a page of projects arrives well inside thirty seconds")
        .expect("the site lists its projects");

    assert!(!page.values.is_empty(), "the site lists at least one project");
    assert_eq!(page.start_at, 0, "an unpaged search starts at the beginning");
}
