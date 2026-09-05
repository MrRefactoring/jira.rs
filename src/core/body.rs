use bytes::Bytes;
use serde_json::Value;

use crate::core::multipart::MultipartBody;

pub const FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

/// The request body, in the shape it goes on the wire.
#[derive(Debug, Clone)]
pub enum Body {
    /// Sent as `application/json`.
    Json(Value),
    /// Sent verbatim, under whatever content type the request declares.
    Text(String),
    /// Sent as `application/x-www-form-urlencoded`. A repeated key is a repeated entry.
    Form(Vec<(String, String)>),
    /// Sent verbatim as bytes.
    Bytes(Bytes),
    /// Sent as `multipart/form-data`, with the boundary the transport picks.
    Multipart(MultipartBody),
}

impl Body {
    /// Anything serialisable, as a JSON body.
    pub fn json<T: serde::Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        Ok(Body::Json(serde_json::to_value(value)?))
    }

    /// Whether the body brings its own content type, so the transport must not name one.
    ///
    /// A bare string is deliberately not one of these. These are JSON APIs, and the endpoints that take a lone string
    /// — an account id, a preference value — want it as a JSON string, quoted, under `application/json`.
    pub(crate) fn carries_own_content_type(&self) -> bool {
        matches!(self, Body::Form(_) | Body::Bytes(_) | Body::Multipart(_))
    }
}

/// Encodes a JSON object the way a form would, dropping what has nothing to send.
///
/// An array becomes a repeated key rather than a comma-joined string, because that is what Jira's column setters
/// read.
pub(crate) fn json_to_form(value: &Value) -> Vec<(String, String)> {
    let Some(object) = value.as_object() else {
        return Vec::new();
    };

    let mut encoded = Vec::new();

    for (key, value) in object {
        match value {
            Value::Null => {}
            Value::Array(items) => {
                for item in items {
                    encoded.push((key.clone(), render_form_scalar(item)));
                }
            }
            other => encoded.push((key.clone(), render_form_scalar(other))),
        }
    }

    encoded
}

fn render_form_scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

/// Whether to declare `application/json` for this request.
///
/// Sent even when there is no body, for any method that could carry one. Some endpoints reject a bodyless `DELETE`
/// with 415 unless the header is present — Jira's remote-link deletes do — and a `Content-Type` on an empty request
/// is inert everywhere else.
pub(crate) fn should_set_json_content_type(body: Option<&Body>, method: &reqwest::Method) -> bool {
    match body {
        Some(body) if body.carries_own_content_type() => false,
        Some(_) => true,
        None => *method != reqwest::Method::GET && *method != reqwest::Method::HEAD,
    }
}
