use crate::harness::{ResourceTracker, cloud, create_test_issue, test_name};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reads_the_account_the_credentials_belong_to() {
    let myself = cloud().myself().get_current_user().send().await.expect("the site knows the caller");

    assert!(myself.account_id.is_some_and(|id| !id.is_empty()), "an authenticated user has an account id");
    assert!(myself.active.unwrap_or(false), "the credentials belong to an active user");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn creates_reads_and_deletes_an_issue() {
    let mut tracker = ResourceTracker::new();
    let summary = test_name("round trip");
    let created = create_test_issue(&mut tracker, Some(&summary)).await;

    let read = cloud()
        .issues()
        .get_issue(&created.key)
        .send()
        .await
        .expect("an issue that was just created can be read back");

    assert_eq!(read.key.as_deref(), Some(created.key.as_str()));

    tracker.cleanup().await;

    let after = cloud().issues().get_issue(&created.key).send().await;

    assert!(after.is_err_and(|error| error.is_not_found()), "the issue is gone once cleanup has run");
}
