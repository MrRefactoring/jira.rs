//! Ported from jira.js/tests/live/cloud/issueRemoteLinks.test.ts.
//!
//! A full write cycle against a fixture issue. Remote links point at things outside Jira — a URL and a label — so
//! nothing is created anywhere else and the whole cycle is contained.
//!
//! The behaviour that makes this API unusual, and that only a live site demonstrates: one endpoint both creates and
//! updates, and which of the two happens is decided by `globalId`. Reusing a `globalId` overwrites; omitting it makes
//! a new link every time. That is a silent difference between "my retry was safe" and "I now have six links".
//!
//! Both deletes are exercised through the client because this API demands a `Content-Type` header even on a bodyless
//! DELETE, and answers 415 to a request that omits one.

use jira::cloud::{GetRemoteIssueLinks, RemoteIssueLink, RemoteIssueLinkRequest, RemoteObject};

use crate::harness::{ResourceTracker, TEST_PROJECT_KEY, cloud, create_test_issue, run_id, test_name};

/// The listing answers with a single object when filtered by `globalId` and an array otherwise, so both shapes are
/// flattened to the same thing.
fn listed(links: GetRemoteIssueLinks) -> Vec<RemoteIssueLink> {
    match links {
        GetRemoteIssueLinks::Variant0(links) => links,
        GetRemoteIssueLinks::RemoteIssueLink(link) => vec![link],
    }
}

fn link_to(url: &str, title: &str, global_id: Option<&str>) -> RemoteIssueLinkRequest {
    RemoteIssueLinkRequest {
        global_id: global_id.map(ToOwned::to_owned),
        relationship: Some("documented by".to_owned()),
        object: Some(RemoteObject { url: url.to_owned(), title: title.to_owned(), ..RemoteObject::default() }),
        ..RemoteIssueLinkRequest::default()
    }
}

async fn links_of(issue_key: &str) -> Vec<RemoteIssueLink> {
    let links = cloud()
        .issue_remote_links()
        .get_remote_issue_links(issue_key)
        .send()
        .await
        .expect("the issue lists its remote links");

    listed(links)
}

/// The remote link lifecycle, end to end.
///
/// The sequence is the point: a fresh issue has none, a create hands back a resolvable identity, a reused `globalId`
/// overwrites the link rather than adding one, an omitted `globalId` adds one, the listing can be filtered down to a
/// single link, and a delete by `globalId` removes exactly that link and leaves the other.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn walks_a_remote_link_through_its_lifecycle() {
    let mut tracker = ResourceTracker::new();
    let issue = create_test_issue(&mut tracker, Some(&test_name("remote links"))).await;
    let global_id = format!("jrs-{}", run_id());

    assert!(links_of(&issue.key).await.is_empty(), "a fresh issue carries no remote links");

    let created = cloud()
        .issue_remote_links()
        .create_or_update_remote_issue_link(
            &issue.key,
            link_to("https://example.com/spec", "The specification", Some(&global_id)),
        )
        .send()
        .await
        .expect("the issue takes a remote link");

    let link_id = created.id.expect("a created link carries an id").to_string();

    assert!(
        created.self_.as_deref().is_some_and(|url| url.starts_with("https://")),
        "a created link carries its own URL: {:?}",
        created.self_,
    );

    let (key, id) = (issue.key.clone(), link_id.clone());

    tracker.defer(move || {
        let (key, id) = (key.clone(), id.clone());

        async move { cloud().issue_remote_links().delete_remote_issue_link_by_id(key, id).send().await }
    });

    let fetched = cloud()
        .issue_remote_links()
        .get_remote_issue_link_by_id(&issue.key, &link_id)
        .send()
        .await
        .expect("the link reads back by id");

    assert_eq!(fetched.global_id.as_deref(), Some(global_id.as_str()));
    assert_eq!(fetched.relationship.as_deref(), Some("documented by"));

    let object = fetched.object.expect("a remote link carries the object it points at");

    assert_eq!(object.url, "https://example.com/spec");
    assert_eq!(object.title, "The specification");

    cloud()
        .issue_remote_links()
        .create_or_update_remote_issue_link(
            &issue.key,
            link_to("https://example.com/spec-v2", "The specification, revised", Some(&global_id)),
        )
        .send()
        .await
        .expect("the same global id is accepted a second time");

    let after_update = links_of(&issue.key).await;

    assert_eq!(after_update.len(), 1, "a reused global id updates in place rather than adding a link");
    assert_eq!(
        after_update[0].object.as_ref().map(|object| object.url.as_str()),
        Some("https://example.com/spec-v2"),
        "the update replaced the remote object",
    );

    let second = cloud()
        .issue_remote_links()
        .create_or_update_remote_issue_link(&issue.key, link_to("https://example.com/other", "Something else", None))
        .send()
        .await
        .expect("a link without a global id is accepted");

    let (key, id) = (issue.key.clone(), second.id.expect("a created link carries an id").to_string());

    tracker.defer(move || {
        let (key, id) = (key.clone(), id.clone());

        async move { cloud().issue_remote_links().delete_remote_issue_link_by_id(key, id).send().await }
    });

    assert_eq!(links_of(&issue.key).await.len(), 2, "an omitted global id makes a new link every time");

    let filtered = cloud()
        .issue_remote_links()
        .get_remote_issue_links(&issue.key)
        .global_id(&global_id)
        .send()
        .await
        .expect("the listing can be filtered by global id");

    let filtered = listed(filtered);

    assert_eq!(filtered.len(), 1, "the filter narrows the listing to the link that carries that global id");
    assert_eq!(filtered[0].global_id.as_deref(), Some(global_id.as_str()));

    cloud()
        .issue_remote_links()
        .delete_remote_issue_link_by_global_id(&issue.key, &global_id)
        .send()
        .await
        .expect("a link can be deleted by its global id");

    let error = cloud()
        .issue_remote_links()
        .get_remote_issue_link_by_id(&issue.key, &link_id)
        .send()
        .await
        .expect_err("a deleted link cannot be read");

    assert!(error.is_not_found(), "{error}");

    let remaining = links_of(&issue.key).await;

    assert_eq!(remaining.len(), 1, "the delete removed exactly the link named and left the other");

    tracker.cleanup().await;
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_remote_links_of_a_missing_issue_as_not_found() {
    let error = cloud()
        .issue_remote_links()
        .get_remote_issue_links(format!("{TEST_PROJECT_KEY}-99999999"))
        .send()
        .await
        .expect_err("an issue that does not exist has no remote links");

    assert!(error.is_not_found(), "{error}");
}
