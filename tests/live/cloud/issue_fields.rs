//! The field catalogue, read-only.
//!
//! A custom field is site-wide: creating one adds a column to every project's configuration, and Jira caps how many a
//! site may have. Trashing and deleting are worse — a deleted field takes its data on every issue with it. None of
//! that belongs in a suite running against a working site, so the write half is pinned only through its error
//! channel, aimed at an id that cannot exist.
//!
//! What is worth asserting is the catalogue itself, because the rest of the API is addressed through it. Field *ids*
//! are what `fields` parameters and JQL clauses ultimately resolve to, and the mapping from a human name to an id is
//! neither stable nor unique — two custom fields may share a name.

use jira::cloud::GetFieldsPaginatedRequestType;

use crate::harness::cloud;

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lists_every_field_on_the_site_each_fully_typed() {
    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");

    assert!(!fields.is_empty(), "a Jira site always has fields");

    for field in &fields {
        assert!(field.id.as_deref().is_some_and(|id| !id.is_empty()), "a field carries an id: {field:?}");
        assert!(field.name.as_deref().is_some_and(|name| !name.is_empty()), "a field carries a name: {field:?}");
        assert!(field.custom.is_some(), "a field says whether it is custom: {field:?}");
        assert!(field.searchable.is_some(), "a field says whether it is searchable: {field:?}");
        assert!(field.orderable.is_some(), "a field says whether it is orderable: {field:?}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn includes_the_system_fields_the_rest_of_the_suite_reads() {
    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");
    let ids: Vec<&str> = fields.iter().filter_map(|field| field.id.as_deref()).collect();

    for expected in ["summary", "description", "issuetype", "project", "status"] {
        assert!(ids.contains(&expected), "the catalogue names {expected}");
    }
}

/// A custom field is addressed by a generated id, never by the name a human gave it — two custom fields may share a
/// name, and only the id resolves.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn distinguishes_custom_fields_by_an_id_that_is_not_their_name() {
    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");

    for field in fields.iter().filter(|field| field.custom == Some(true)) {
        let id = field.id.as_deref().expect("a custom field carries an id");

        assert!(is_custom_field_id(id), "a custom field id is customfield_ and digits: {id}");
        assert!(
            field.schema.as_ref().and_then(|schema| schema.custom_id).is_some(),
            "a custom field's schema carries its numeric id: {field:?}",
        );
    }

    for field in fields.iter().filter(|field| field.custom != Some(true)) {
        let id = field.id.as_deref().expect("a system field carries an id");

        assert!(!id.starts_with("customfield_"), "a system field is not in the custom namespace: {id}");
    }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_what_each_field_holds_through_its_schema() {
    let fields = cloud().issue_fields().get_fields().send().await.expect("the site lists its fields");

    let summary = fields
        .iter()
        .find(|field| field.id.as_deref() == Some("summary"))
        .and_then(|field| field.schema.as_ref())
        .expect("the summary field declares a schema");

    assert_eq!(summary.r#type, "string");
    assert_eq!(summary.items, None, "a scalar field names no item type");

    let issuetype = fields
        .iter()
        .find(|field| field.id.as_deref() == Some("issuetype"))
        .and_then(|field| field.schema.as_ref())
        .expect("the issuetype field declares a schema");

    assert_eq!(issuetype.r#type, "issuetype", "a schema names the shape it holds, not just 'object'");
}

/// The paginated listing is administrator-only, so a token without *Administer Jira* must be refused in a way the
/// caller can recognise rather than left to guess at an untyped failure.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn pages_the_custom_field_listing_for_an_admin_or_fails_typed() {
    let page = match cloud().issue_fields().get_fields_paginated().max_results(5).send().await {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "the paginated listing is refused by rights, not by shape: {error}");

            return;
        }
    };

    assert_eq!(page.max_results, 5, "the page size asked for is the page size returned");
    assert_eq!(page.start_at, 0, "an unoffset request starts at the beginning");
    assert!(page.values.len() <= 5, "a page holds no more than it says it does");
    assert!(page.is_last || page.values.len() == 5, "a page that is not the last one is full");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn filters_the_paginated_listing_to_custom_fields_only() {
    let listing = cloud()
        .issue_fields()
        .get_fields_paginated()
        .r#type([GetFieldsPaginatedRequestType::Custom])
        .max_results(50)
        .send()
        .await;

    let page = match listing {
        Ok(page) => page,
        Err(error) => {
            assert!(error.is_forbidden(), "the paginated listing is refused by rights, not by shape: {error}");

            return;
        }
    };

    for field in &page.values {
        assert!(is_custom_field_id(&field.id), "the custom filter returns only the custom namespace: {}", field.id);
    }
}

/// The destructive path, proven through its error channel and never aimed at a field that exists.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn fails_typed_on_the_destructive_path() {
    let error = cloud()
        .issue_fields()
        .delete_custom_field("customfield_99999999")
        .send()
        .await
        .expect_err("a custom field that does not exist cannot be deleted");

    assert!(
        error.is_not_found() || error.is_forbidden() || error.status() == Some(400),
        "a refused delete is typed: {error}",
    );
}

fn is_custom_field_id(id: &str) -> bool {
    let Some(digits) = id.strip_prefix("customfield_") else { return false };

    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}
