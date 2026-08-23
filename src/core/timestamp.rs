use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Reads a timestamp the specification calls a string and the API sometimes sends as a number.
///
/// Jira declares every `date-time` field as a string, and most endpoints send one. The bulk queue does not: it
/// answers `"created": 1787521555310`, epoch milliseconds as a JSON integer. `jira.js` absorbs that because zod's
/// `coerce.date` takes either; nothing in serde does, so a field that believes the document cannot read the task it
/// was just handed.
///
/// A number is kept as its digits rather than rendered as an instant. Turning it into an ISO string would mean
/// choosing a format the API never used, and the value is still exactly what arrived.
pub fn deserialize_timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text)),
        Some(Value::Number(number)) => Ok(Some(number.to_string())),
        Some(other) => Err(serde::de::Error::custom(format!("a timestamp is a string or a number, got {other}"))),
    }
}

/// The same, for a field the specification says is always present.
pub fn deserialize_required_timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    match Value::deserialize(deserializer)? {
        Value::String(text) => Ok(text),
        Value::Number(number) => Ok(number.to_string()),
        other => Err(serde::de::Error::custom(format!("a timestamp is a string or a number, got {other}"))),
    }
}
