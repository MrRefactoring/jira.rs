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
