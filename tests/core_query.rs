use jira::QueryValue;
use jira::core::build_url_with_search_params;

const BASE: &str = "https://acme.atlassian.net/rest/api/3/search";

fn params(entries: Vec<(&str, QueryValue)>) -> Vec<(String, QueryValue)> {
    entries.into_iter().map(|(name, value)| (name.to_owned(), value)).collect()
}

#[test]
fn renders_scalars_as_they_are() {
    assert_eq!(QueryValue::from("page"), QueryValue::Scalar("page".to_owned()));
    assert_eq!(QueryValue::from(42), QueryValue::Scalar("42".to_owned()));
    assert_eq!(QueryValue::from(0), QueryValue::Scalar("0".to_owned()));
    assert_eq!(QueryValue::from(false), QueryValue::Scalar("false".to_owned()));
}

#[test]
fn drops_what_has_nothing_to_send_so_the_key_never_reaches_the_query() {
    assert!(QueryValue::from(None::<String>).is_skip());
    assert!(QueryValue::from(serde_json::Value::Null).is_skip());
    assert_eq!(build_url_with_search_params(BASE, &params(vec![("a", QueryValue::from(None::<i32>))])), BASE);
}

#[test]
fn json_encodes_an_object() {
    let url =
        build_url_with_search_params(BASE, &params(vec![("filter", QueryValue::from(serde_json::json!({ "a": 1 })))]));

    assert_eq!(url, format!("{BASE}?filter=%7B%22a%22%3A1%7D"));
}

#[test]
fn returns_the_url_untouched_when_there_are_no_params() {
    assert_eq!(build_url_with_search_params(BASE, &[]), BASE);
}

#[test]
fn returns_the_url_untouched_when_every_param_is_empty() {
    let url = build_url_with_search_params(BASE, &params(vec![("a", QueryValue::Skip), ("b", QueryValue::Skip)]));

    assert_eq!(url, BASE);
}

#[test]
fn appends_params_with_a_question_mark() {
    let url = build_url_with_search_params(BASE, &params(vec![("maxResults", QueryValue::from(25))]));

    assert_eq!(url, format!("{BASE}?maxResults=25"));
}

#[test]
fn appends_with_an_ampersand_when_the_url_already_carries_a_query() {
    let url =
        build_url_with_search_params(&format!("{BASE}?cursor=x"), &params(vec![("maxResults", QueryValue::from(25))]));

    assert_eq!(url, format!("{BASE}?cursor=x&maxResults=25"));
}

#[test]
fn repeats_the_key_for_each_array_item_rather_than_joining() {
    let url =
        build_url_with_search_params(BASE, &params(vec![("fields", QueryValue::from(vec!["summary", "status"]))]));

    assert_eq!(url, format!("{BASE}?fields=summary&fields=status"));
}

#[test]
fn skips_an_empty_list_entirely() {
    let url = build_url_with_search_params(BASE, &params(vec![("fields", QueryValue::List(Vec::new()))]));

    assert_eq!(url, BASE);
}

#[test]
fn percent_encodes_values() {
    let url = build_url_with_search_params(BASE, &params(vec![("jql", QueryValue::from("type = Task"))]));

    assert_eq!(url, format!("{BASE}?jql=type+%3D+Task"));
}
