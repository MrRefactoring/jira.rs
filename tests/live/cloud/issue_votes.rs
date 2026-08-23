//! Ported from jira.js/tests/live/cloud/issueVotes.test.ts.
//!
//! A full write cycle that is genuinely safe: votes live entirely inside a disposable fixture issue, and both halves
//! of the mutation are exercised, so the suite leaves nothing behind even before teardown runs.

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, test_name};

/// The vote lifecycle, end to end.
///
/// Worth recording, because it contradicts the documented Jira Server behaviour and the intuition that comes with it:
/// on Cloud the reporter *can* vote for their own issue. The fixture issue is reported by the authenticating account
/// and the vote is accepted. Any caller carrying over a "reporters cannot vote" guard from Server is wrong here.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_vote_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("votes"))).await;

    let fresh = cloud().issue_votes().get_votes(&issue.key).send().await.expect("a fresh issue reports its votes");

    assert_eq!(fresh.votes, Some(0));
    assert_eq!(fresh.has_voted, Some(false));
    assert!(
        fresh.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "the vote count carries its own URL: {:?}",
        fresh.self_,
    );

    cloud().issue_votes().add_vote(&issue.key).send().await.expect("the reporter may vote for their own issue");

    let key = issue.key.clone();

    tracker.defer(move || {
        let key = key.clone();

        async move { cloud().issue_votes().remove_vote(key).send().await }
    });

    let voted = cloud().issue_votes().get_votes(&issue.key).send().await.expect("the vote count reads back");

    assert_eq!(voted.votes, Some(1), "the vote is observable on the next read");
    assert_eq!(voted.has_voted, Some(true));

    cloud().issue_votes().add_vote(&issue.key).send().await.expect("a repeated vote is accepted");

    let again = cloud().issue_votes().get_votes(&issue.key).send().await.expect("the vote count reads back");

    assert_eq!(again.votes, Some(1), "a repeated vote is idempotent rather than cumulative");

    cloud().issue_votes().remove_vote(&issue.key).send().await.expect("the vote can be withdrawn");

    let withdrawn = cloud().issue_votes().get_votes(&issue.key).send().await.expect("the vote count reads back");

    assert_eq!(withdrawn.votes, Some(0));
    assert_eq!(withdrawn.has_voted, Some(false));

    cloud()
        .issue_votes()
        .remove_vote(&issue.key)
        .send()
        .await
        .expect("removing a vote that is no longer there is accepted rather than a not-found");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_votes_of_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_votes()
        .get_votes(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist has no votes");

    assert!(error.is_not_found(), "{error}");
}
