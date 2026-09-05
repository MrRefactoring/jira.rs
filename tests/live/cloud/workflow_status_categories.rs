//! Ported from jira.js/tests/live/cloud/workflowStatusCategories.test.ts.
//!
//! Status categories are the fixed four-value vocabulary every Jira workflow ultimately maps onto, so this is a rare
//! endpoint where the exact contents can be asserted rather than merely their shape — and where a drift would break
//! every board and report built on top of them.

use jira::cloud::StatusCategory;

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn returns_the_categories_jira_defines_each_fully_typed() {
    let categories = site_status_categories().await;

    for category in &categories {
        assert!(category.id.is_some(), "a category carries an id: {category:?}");
        assert!(category.key.as_deref().is_some_and(|key| !key.is_empty()), "{category:?}");
        assert!(category.name.as_deref().is_some_and(|name| !name.is_empty()), "{category:?}");
        assert!(category.color_name.as_deref().is_some_and(|color| !color.is_empty()), "{category:?}");
        assert!(category.self_.as_deref().is_some_and(|url| url.starts_with("https://")), "{category:?}");
    }

    let mut keys: Vec<&str> = categories.iter().filter_map(|category| category.key.as_deref()).collect();

    keys.sort_unstable();

    assert_eq!(
        keys,
        ["done", "indeterminate", "new", "undefined"],
        "the vocabulary every workflow maps onto is fixed, and this is it",
    );
}

/// Two ways of addressing the same category, and both have to give the record the listing did.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn resolves_a_single_category_by_id_and_by_key_alike() {
    let categories = site_status_categories().await;

    let done = categories
        .iter()
        .find(|category| category.key.as_deref() == Some("done"))
        .expect("Jira defines a done category");

    let id = done.id.expect("a status category carries an id");

    let by_id = cloud()
        .workflow_status_categories()
        .get_status_category(id.to_string())
        .send()
        .await
        .expect("a category reads back by id");

    let by_key = cloud()
        .workflow_status_categories()
        .get_status_category("done")
        .send()
        .await
        .expect("a category reads back by key");

    assert_eq!(by_id.id, done.id);
    assert_eq!(by_id.key, done.key);
    assert_eq!(by_id.name, done.name);
    assert_eq!(by_id.color_name, done.color_name);
    assert_eq!(by_id.self_, done.self_, "reading one category gives the same record the listing did");
    assert_eq!(by_key.id, done.id, "the id and the key address the same category");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn surfaces_an_unknown_category_as_not_found() {
    let error = cloud()
        .workflow_status_categories()
        .get_status_category("no-such-category")
        .send()
        .await
        .expect_err("a status category that does not exist cannot be read");

    assert!(error.is_not_found(), "{error}");
}

async fn site_status_categories() -> Vec<StatusCategory> {
    cloud().workflow_status_categories().get_status_categories().send().await.expect("the site lists its categories")
}
