use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    match Option::<Value>::deserialize(deserializer)? {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text)),
        Some(Value::Number(number)) => Ok(Some(number.to_string())),
        Some(other) => Err(serde::de::Error::custom(format!("a timestamp is a string or a number, got {other}"))),
    }
}

pub fn deserialize_required_timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    match Value::deserialize(deserializer)? {
        Value::String(text) => Ok(text),
        Value::Number(number) => Ok(number.to_string()),
        other => Err(serde::de::Error::custom(format!("a timestamp is a string or a number, got {other}"))),
    }
}
