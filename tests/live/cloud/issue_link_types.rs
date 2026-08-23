//! The site's link types, read-only.
//!
//! Link types are site-wide configuration, not project state: creating one adds a permanent option to every issue on
//! the tenant, and deleting one silently drops every link that used it. So the write half is deliberately not
//! exercised here — the read half is asserted fully, and the writes are pinned only to the extent that they fail
//! typed without admin rights.

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_the_site_link_types_each_with_both_directional_phrasings() {
    let types = cloud().issue_link_types().get_issue_link_types().send().await.expect("the site lists its link types");
    let types = types.issue_link_types.expect("the listing carries a collection of link types");

    assert!(!types.is_empty(), "a Jira site always ships with link types");

    for link_type in &types {
        let id = link_type.id.as_deref().expect("a link type carries an id");

        assert!(id.chars().all(|character| character.is_ascii_digit()), "an id is digits: {id}");
        assert!(link_type.name.as_deref().is_some_and(|name| !name.is_empty()), "{link_type:?}");
        assert!(link_type.inward.as_deref().is_some_and(|inward| !inward.is_empty()), "{link_type:?}");
        assert!(link_type.outward.as_deref().is_some_and(|outward| !outward.is_empty()), "{link_type:?}");
        assert!(link_type.self_.as_deref().is_some_and(|url| url.starts_with("https://")), "{link_type:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn includes_the_relates_type_every_jira_site_ships_with() {
    let types = cloud().issue_link_types().get_issue_link_types().send().await.expect("the site lists its link types");
    let types = types.issue_link_types.unwrap_or_default();

    let relates = types
        .iter()
        .find(|link_type| link_type.name.as_deref() == Some("Relates"))
        .expect("every site ships with the Relates link type");

    assert_eq!(relates.inward.as_deref(), Some("relates to"));
    assert_eq!(relates.outward.as_deref(), Some("relates to"));
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_type_by_id_identical_to_its_listing_entry() {
    let types = cloud().issue_link_types().get_issue_link_types().send().await.expect("the site lists its link types");
    let types = types.issue_link_types.unwrap_or_default();
    let sample = types.first().expect("a site has at least one link type");
    let id = sample.id.clone().expect("a link type carries an id");

    let fetched = cloud()
        .issue_link_types()
        .get_issue_link_type(&id)
        .send()
        .await
        .expect("a link type from the listing reads back by id");

    assert_eq!(fetched.id, sample.id);
    assert_eq!(fetched.name, sample.name);
    assert_eq!(fetched.inward, sample.inward);
    assert_eq!(fetched.outward, sample.outward);
    assert_eq!(fetched.self_, sample.self_, "reading one type gives the same record the listing did");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_type_id_as_not_found() {
    let error = cloud()
        .issue_link_types()
        .get_issue_link_type("99999999")
        .send()
        .await
        .expect_err("a link type that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

/// The destructive path, proven through its error channel and never aimed at a type that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_link_types()
        .delete_issue_link_type("99999999")
        .send()
        .await
        .expect_err("a link type that does not exist cannot be deleted");

    assert!(error.is_not_found() || error.is_forbidden(), "a refused delete is typed: {error}");
}
