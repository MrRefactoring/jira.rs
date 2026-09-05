//! Ported from jira.js/tests/live/cloud/jql.test.ts.
//!
//! Read-only. This is the machinery a query builder is made of: what fields exist, what values they take, and
//! whether a string is valid before it is run. The distinction the suite pins is that parsing is *not* the same as
//! searching — `parse_jql_queries` reports on a query without executing it, and its validation mode decides whether
//! a suspicious but legal query is an error, a warning, or neither.

use jira::cloud::{JqlQueriesToParse, ParseJqlQueriesRequestValidation};

use crate::harness::{TEST_PROJECT_KEY, cloud};

fn queries_of(queries: &[&str]) -> JqlQueriesToParse {
    JqlQueriesToParse { queries: queries.iter().map(|query| (*query).to_owned()).collect() }
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn describes_the_fields_a_query_can_be_built_from() {
    let data = cloud().jql().get_auto_complete().send().await.expect("the site describes its query vocabulary");

    let fields = data.visible_field_names.expect("the reference data names the visible fields");
    let functions = data.visible_function_names.expect("the reference data names the visible functions");
    let reserved = data.jql_reserved_words.expect("the reference data names the reserved words");

    assert!(!fields.is_empty(), "a site always has fields to query by");
    assert!(!functions.is_empty(), "a site always has functions to query with");
    assert!(!reserved.is_empty(), "JQL always reserves some words");

    assert!(
        fields.iter().all(|field| field.value.as_deref().is_some_and(|value| !value.is_empty())),
        "every field carries the name a query would spell it by",
    );
    assert!(fields.iter().all(|field| field.searchable.is_some()), "every field says whether it can be searched");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn includes_the_fields_the_rest_of_the_suite_queries_by() {
    let data = cloud().jql().get_auto_complete().send().await.expect("the site describes its query vocabulary");

    let names: Vec<String> =
        data.visible_field_names.unwrap_or_default().into_iter().filter_map(|field| field.value).collect();

    for expected in ["project", "summary", "status", "created"] {
        assert!(names.iter().any(|name| name == expected), "`{expected}` is a field a query can be built from");
    }
}

/// What a query builder calls as the user types: a field name and a partial value, answered with candidates.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn suggests_values_for_a_field_as_a_user_types() {
    let suggestions = cloud()
        .jql()
        .get_field_auto_complete_for_query_string()
        .field_name("project")
        .field_value(&TEST_PROJECT_KEY[..4])
        .send()
        .await
        .expect("a partial field value is answered with suggestions");

    let results = suggestions.results.expect("the answer carries its results");

    assert!(!results.is_empty(), "the test project starts with that prefix, so something matches it");
    assert!(
        results.iter().all(|result| {
            result.value.as_deref().is_some_and(|value| !value.is_empty())
                && result.display_name.as_deref().is_some_and(|name| !name.is_empty())
        }),
        "every suggestion carries both what to insert and what to show",
    );
}

/// Parsing reports on a query rather than running it, and hands back the structure a builder can edit.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn parses_a_valid_query_into_its_structure_without_running_it() {
    let parsed = cloud()
        .jql()
        .parse_jql_queries(
            ParseJqlQueriesRequestValidation::Strict,
            queries_of(&[&format!("project = {TEST_PROJECT_KEY} ORDER BY created DESC")]),
        )
        .send()
        .await
        .expect("a valid query parses");

    assert_eq!(parsed.queries.len(), 1, "one answer per query");

    let query = &parsed.queries[0];

    assert!(query.errors.as_deref().unwrap_or_default().is_empty(), "a valid query has nothing to report");
    assert!(query.structure.is_some(), "a parsed query carries its structure");

    let order_by = query
        .structure
        .as_ref()
        .and_then(|structure| structure.order_by.as_ref())
        .expect("the ORDER BY clause is part of the structure");

    assert_eq!(order_by.fields[0].field.name, "created");
    assert!(
        query.structure.as_ref().is_some_and(|structure| structure.r#where.is_some()),
        "the WHERE clause is part of the structure",
    );
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn reports_errors_for_a_malformed_query_instead_of_failing_the_call() {
    let parsed = cloud()
        .jql()
        .parse_jql_queries(ParseJqlQueriesRequestValidation::Strict, queries_of(&["project = \"unterminated"]))
        .send()
        .await
        .expect("parsing a broken query is still a successful call");

    let query = &parsed.queries[0];

    assert!(!query.errors.as_deref().unwrap_or_default().is_empty(), "the breakage is reported in the answer");
    assert!(query.structure.is_none(), "a query that does not parse has no structure");
}

/// The same suspicious query is an error under `strict` and unremarkable under `none` — which is the difference
/// between a builder that refuses a typo and one that lets it through.
#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn lets_the_validation_mode_decide_how_strict_the_answer_is() {
    let suspicious = format!("project = {TEST_PROJECT_KEY} AND nosuchfield = 1");

    let strict = cloud()
        .jql()
        .parse_jql_queries(ParseJqlQueriesRequestValidation::Strict, queries_of(&[&suspicious]))
        .send()
        .await
        .expect("strict validation is accepted");

    let none = cloud()
        .jql()
        .parse_jql_queries(ParseJqlQueriesRequestValidation::None, queries_of(&[&suspicious]))
        .send()
        .await
        .expect("validation can be turned off");

    assert!(
        !strict.queries[0].errors.as_deref().unwrap_or_default().is_empty(),
        "under `strict` an unknown field is an error",
    );
    assert!(
        none.queries[0].errors.as_deref().unwrap_or_default().is_empty(),
        "under `none` the same query is reported clean",
    );
    assert!(none.queries[0].structure.is_some(), "and it still parses into a structure");
}

#[tokio::test]
#[ignore = "live: needs a Jira site"]
async fn parses_several_queries_in_one_call_independently() {
    let parsed = cloud()
        .jql()
        .parse_jql_queries(
            ParseJqlQueriesRequestValidation::Strict,
            queries_of(&[&format!("project = {TEST_PROJECT_KEY}"), "project = \"unterminated"]),
        )
        .send()
        .await
        .expect("several queries parse in one call");

    assert_eq!(parsed.queries.len(), 2, "one answer per query, in the order they were given");
    assert!(
        parsed.queries[0].errors.as_deref().unwrap_or_default().is_empty(),
        "the valid query is unaffected by the broken one",
    );
    assert!(
        !parsed.queries[1].errors.as_deref().unwrap_or_default().is_empty(),
        "the broken query is reported on its own",
    );
}
