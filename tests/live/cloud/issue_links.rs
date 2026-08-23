//! Links between issues, created, read and removed.
//!
//! Safe to run: links exist only between issues this suite created, and the delete half is exercised rather than left
//! to teardown.
//!
//! The endpoint has a quirk that shapes the whole suite — creating a link answers with nothing useful, no id and no
//! location. The only way to reach the link just made is to read it back off one of the issues, and every caller has
//! to do the same. That indirection is the thing worth pinning.

use jira::cloud::{IssueLinkType, LinkIssueRequest, LinkedIssue};

use crate::harness::{ResourceTracker, cloud, create_test_issue, test_name};

/// The full cycle, walked in one test because each step needs the link the step before it made.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_link_through_creation_reading_and_deletion() {
    let mut tracker = ResourceTracker::new();
    let inward = create_test_issue(&mut tracker, Some(&test_name("link target"))).await;
    let outward = create_test_issue(&mut tracker, Some(&test_name("link source"))).await;
    let link_type = a_link_type().await;
    let type_name = link_type.name.clone().expect("a link type carries a name");

    cloud()
        .issue_links()
        .link_issues(LinkIssueRequest {
            r#type: link_type,
            inward_issue: LinkedIssue { key: Some(inward.key.clone()), ..LinkedIssue::default() },
            outward_issue: LinkedIssue { key: Some(outward.key.clone()), ..LinkedIssue::default() },
            comment: None,
        })
        .send()
        .await
        .expect("two issues in the same project can be linked");

    let source_links = links_on(&outward.key).await;
    let target_links = links_on(&inward.key).await;

    assert_eq!(source_links.len(), 1, "the source issue carries exactly the link just made");
    assert_eq!(target_links.len(), 1, "the target issue carries exactly the link just made");

    assert_eq!(
        source_links[0].pointer("/inwardIssue/key").and_then(serde_json::Value::as_str),
        Some(inward.key.as_str()),
        "the source sees the other end as its inward issue",
    );
    assert_eq!(
        target_links[0].pointer("/outwardIssue/key").and_then(serde_json::Value::as_str),
        Some(outward.key.as_str()),
        "the target sees the other end as its outward issue",
    );

    let link_id = id_of(&source_links[0]);

    assert_eq!(link_id, id_of(&target_links[0]), "both issues name one link, not two");

    let link = cloud().issue_links().get_issue_link(&link_id).send().await.expect("the link reads back by id");

    assert_eq!(link.id.as_deref(), Some(link_id.as_str()));
    assert_eq!(link.inward_issue.and_then(|issue| issue.key).as_deref(), Some(inward.key.as_str()));
    assert_eq!(link.outward_issue.and_then(|issue| issue.key).as_deref(), Some(outward.key.as_str()));
    assert_eq!(link.r#type.and_then(|link_type| link_type.name), Some(type_name), "the link reports its own type");

    cloud().issue_links().delete_issue_link(&link_id).send().await.expect("a link can be removed");

    let gone =
        cloud().issue_links().get_issue_link(&link_id).send().await.expect_err("a deleted link cannot be read back");

    assert!(gone.is_not_found(), "{gone}");
    assert!(links_on(&outward.key).await.is_empty(), "the delete removes the link from the issues as well");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn rejects_a_link_to_an_issue_that_does_not_exist() {
    let mut tracker = ResourceTracker::new();
    let outward = create_test_issue(&mut tracker, Some(&test_name("link source"))).await;

    let error = cloud()
        .issue_links()
        .link_issues(LinkIssueRequest {
            r#type: a_link_type().await,
            inward_issue: LinkedIssue { key: Some("NOSUCH-1".to_owned()), ..LinkedIssue::default() },
            outward_issue: LinkedIssue { key: Some(outward.key.clone()), ..LinkedIssue::default() },
            comment: None,
        })
        .send()
        .await
        .expect_err("an issue that does not exist cannot be one end of a link");

    assert!(error.status().is_some_and(|status| status >= 400), "{error}");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_link_id_as_not_found() {
    let error = cloud()
        .issue_links()
        .get_issue_link("99999999")
        .send()
        .await
        .expect_err("a link that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

/// `Relates` where the site has it, and whatever it does have otherwise — the suite cares that a link type exists,
/// not which one.
async fn a_link_type() -> IssueLinkType {
    let types = cloud().issue_link_types().get_issue_link_types().send().await.expect("the site lists its link types");
    let mut types = types.issue_link_types.unwrap_or_default();

    let relates = types.iter().position(|link_type| link_type.name.as_deref() == Some("Relates"));

    match relates {
        Some(at) => types.swap_remove(at),
        None => {
            assert!(!types.is_empty(), "a site has at least one link type to link with");

            types.swap_remove(0)
        }
    }
}

/// The links an issue carries, straight off the `issuelinks` field — the only route to a link's id.
async fn links_on(key: &str) -> Vec<serde_json::Value> {
    let issue = cloud()
        .issues()
        .get_issue(key)
        .fields(["issuelinks"])
        .send()
        .await
        .expect("an issue reads back with its links");

    issue
        .fields
        .as_ref()
        .and_then(|fields| fields.get("issuelinks"))
        .and_then(serde_json::Value::as_array)
        .cloned()
        .expect("an issue carries its links under issuelinks")
}

fn id_of(link: &serde_json::Value) -> String {
    link.get("id").and_then(serde_json::Value::as_str).expect("a link on an issue carries an id").to_owned()
}
