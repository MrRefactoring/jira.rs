//! Labels are site-wide and derived state: nothing creates a label directly, they come into existence by being put on
//! an issue. So the suite makes one for real and asserts it surfaces in the global listing, and that paging behaves.

use jira::cloud::IssueFields;

use crate::harness::{ResourceTracker, cloud, create_issue_with, poll_until, run_id, test_issue_fields, test_name};

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_a_well_formed_page_of_site_wide_labels() {
    let page = cloud().labels().get_all_labels().max_results(50).send().await.expect("the site lists its labels");

    assert!(page.values.is_some(), "a page carries its values");
    assert!(page.total.is_some(), "a page carries how many labels there are in total");
    assert!(page.is_last.is_some(), "a page says whether it is the last one");
    assert_eq!(page.start_at, Some(0), "an unasked-for offset is the beginning");
    assert!(
        page.values.as_deref().unwrap_or_default().iter().all(|label| !label.is_empty()),
        "a label is never the empty string",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn eventually_lists_the_label_just_put_on_an_issue() {
    // Jira labels may not contain whitespace; run-scoped so concurrent runs cannot collide.
    let label = format!("jrs{}label", run_id());
    let mut tracker = ResourceTracker::new();

    create_issue_with(
        &mut tracker,
        IssueFields { labels: Some(vec![label.clone()]), ..test_issue_fields(test_name("labelled")) },
    )
    .await;

    let values = poll_until("the label to reach the site-wide listing", || async {
        let page = cloud()
            .labels()
            .get_all_labels()
            .max_results(1000)
            .send()
            .await
            .expect("the label listing stays readable while indexing catches up");

        page.values.filter(|values| values.contains(&label))
    })
    .await;

    assert!(values.contains(&label), "the label an issue carries is a label of the site");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn honours_max_results_and_start_at() {
    let first = cloud().labels().get_all_labels().max_results(2).send().await.expect("the first page is readable");

    assert_eq!(first.max_results, Some(2), "the page echoes the limit asked for");
    assert!(first.values.as_deref().unwrap_or_default().len() <= 2, "the limit is honoured, not merely echoed");

    if first.total.unwrap_or_default() > 2 {
        let second = cloud()
            .labels()
            .get_all_labels()
            .max_results(2)
            .start_at(2)
            .send()
            .await
            .expect("the second page is readable");

        assert_eq!(second.start_at, Some(2), "the page echoes the offset asked for");
        assert_ne!(second.values, first.values, "an offset moves the window rather than repeating it");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_is_last_truthfully_at_the_end_of_the_listing() {
    let all = cloud().labels().get_all_labels().max_results(1000).send().await.expect("the whole listing is readable");

    if all.total.unwrap_or_default() <= 1000 {
        assert_eq!(all.is_last, Some(true), "a page holding every label is the last one");
    }
}
