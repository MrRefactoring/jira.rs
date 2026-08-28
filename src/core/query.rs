use serde_json::Value;

/// One query-string value, in the shape the API expects it on the wire.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryValue {
    /// A string, number or boolean, already rendered.
    Scalar(String),
    /// Repeated key — `columns=summary&columns=status`, which is what Jira's list parameters read.
    List(Vec<String>),
    /// Anything structured, sent as JSON.
    Json(Value),
    /// Nothing to send. The parameter is left out of the URL entirely.
    Skip,
}

impl QueryValue {
    pub fn is_skip(&self) -> bool {
        matches!(self, QueryValue::Skip)
    }
}

impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        QueryValue::Scalar(value)
    }
}

impl From<&str> for QueryValue {
    fn from(value: &str) -> Self {
        QueryValue::Scalar(value.to_owned())
    }
}

impl From<bool> for QueryValue {
    fn from(value: bool) -> Self {
        QueryValue::Scalar(value.to_string())
    }
}

macro_rules! query_value_from_number {
    ($($type:ty),*) => {
        $(
            impl From<$type> for QueryValue {
                fn from(value: $type) -> Self {
                    QueryValue::Scalar(value.to_string())
                }
            }
        )*
    };
}

query_value_from_number!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl<T: Into<QueryValue>> From<Option<T>> for QueryValue {
    fn from(value: Option<T>) -> Self {
        value.map_or(QueryValue::Skip, Into::into)
    }
}

impl From<Vec<String>> for QueryValue {
    fn from(value: Vec<String>) -> Self {
        QueryValue::List(value)
    }
}

impl From<Vec<&str>> for QueryValue {
    fn from(value: Vec<&str>) -> Self {
        QueryValue::List(value.into_iter().map(ToOwned::to_owned).collect())
    }
}

impl From<Value> for QueryValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => QueryValue::Skip,
            Value::String(text) => QueryValue::Scalar(text),
            Value::Bool(flag) => QueryValue::Scalar(flag.to_string()),
            Value::Number(number) => QueryValue::Scalar(number.to_string()),
            other => QueryValue::Json(other),
        }
    }
}

impl QueryValue {
    /// The query value for anything that can be written as JSON.
    ///
    /// The fallback the generated operations use for a parameter whose type is neither a string nor a number: an
    /// enum, a list of them, a structure the specification described inline. A list becomes a repeated key, because
    /// that is what Jira's list parameters read; anything with a shape of its own goes as JSON, which is what the
    /// endpoints taking one expect.
    pub fn from_serializable<T: serde::Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        Ok(QueryValue::from_json(serde_json::to_value(value)?))
    }

    fn from_json(value: Value) -> Self {
        match value {
            Value::Null => QueryValue::Skip,
            Value::String(text) => QueryValue::Scalar(text),
            Value::Bool(flag) => QueryValue::Scalar(flag.to_string()),
            Value::Number(number) => QueryValue::Scalar(number.to_string()),
            Value::Array(items) => QueryValue::List(items.into_iter().map(render_scalar).collect()),
            object => QueryValue::Json(object),
        }
    }
}

/// One list item, as the query string spells it: a string as itself, anything else as its JSON.
fn render_scalar(value: Value) -> String {
    match value {
        Value::String(text) => text,
        other => other.to_string(),
    }
}

/// The header value for anything that can be written as JSON.
///
/// A header is text, so a string goes as itself; everything else is written the way JSON would write it, which for
/// the enums Jira declares as header parameters is the value the API named.
#[allow(dead_code)]
pub fn header_value<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    Ok(render_scalar(serde_json::to_value(value)?))
}

/// Appends the query string to `base_url`, leaving it untouched when there is nothing to append.
///
/// An array becomes a repeated key rather than a comma-joined string, because that is what Jira's list parameters
/// read: `columns=summary&columns=status` sets two columns, `columns=summary,status` sets one with a comma in its
/// name.
pub fn build_url_with_search_params(base_url: &str, params: &[(String, QueryValue)]) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());
    let mut any = false;

    for (key, value) in params {
        match value {
            QueryValue::Skip => {}
            QueryValue::Scalar(text) => {
                serializer.append_pair(key, text);
                any = true;
            }
            QueryValue::List(items) => {
                for item in items {
                    serializer.append_pair(key, item);
                    any = true;
                }
            }
            QueryValue::Json(json) => {
                serializer.append_pair(key, &json.to_string());
                any = true;
            }
        }
    }

    if !any {
        return base_url.to_owned();
    }

    let query = serializer.finish();
    let separator = if base_url.contains('?') { '&' } else { '?' };

    format!("{base_url}{separator}{query}")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let url = build_url_with_search_params(
            BASE,
            &params(vec![("filter", QueryValue::from(serde_json::json!({ "a": 1 })))]),
        );

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
        let url = build_url_with_search_params(
            &format!("{BASE}?cursor=x"),
            &params(vec![("maxResults", QueryValue::from(25))]),
        );

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
}
