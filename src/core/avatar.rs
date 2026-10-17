use serde_json::Value;

use crate::core::{Error, Result, SchemaMismatchIssue, SchemaMismatchReport};

pub(super) fn parse(text: &str, endpoint: &str) -> Result<Value> {
    let lower = text.to_ascii_lowercase();
    let json = lower
        .find("<textarea")
        .filter(|start| lower.as_bytes().get(start + 9).is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b'>'))
        .and_then(|start| lower[start..].find('>').map(|offset| start + offset + 1))
        .and_then(|start| lower[start..].find("</textarea>").map(|length| &text[start..start + length]))
        .unwrap_or(text);
    let mut value: Value = serde_json::from_str(json).map_err(|source| Error::SchemaMismatch {
        report: Box::new(SchemaMismatchReport {
            endpoint: endpoint.to_owned(),
            issues: vec![SchemaMismatchIssue {
                path: String::new(),
                expected: "JSON in a textarea".to_owned(),
                received: "malformed response".to_owned(),
            }],
        }),
        source: Some(source),
    })?;

    if let Some(object) = value.as_object_mut() {
        for key in ["cropperWidth", "cropperOffsetX", "cropperOffsetY"] {
            if let Some(Value::String(text)) = object.get(key) {
                let digits = text.strip_prefix('-').unwrap_or(text);

                if !digits.is_empty()
                    && digits.bytes().all(|byte| byte.is_ascii_digit())
                    && let Ok(number) = text.parse::<i64>()
                {
                    object.insert(key.to_owned(), Value::from(number));
                }
            }
        }

        if let Some(value) = object.remove("isCroppingNeeded") {
            let value = match value.as_str() {
                Some("true") => Value::Bool(true),
                Some("false") => Value::Bool(false),
                _ => value,
            };

            object.insert("needsCropping".to_owned(), value);
        }
    }

    Ok(value)
}
