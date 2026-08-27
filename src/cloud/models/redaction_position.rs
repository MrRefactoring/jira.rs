// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Represents the position of the redaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RedactionPosition {
    /// The ADF pointer indicating the position of the text to be redacted. This is only required when redacting from rich text(ADF) fields. For plain text fields, this field can be omitted.
    #[serde(rename = "adfPointer", default, skip_serializing_if = "Option::is_none")]
    pub adf_pointer: Option<String>,
    /// The text which will be redacted, encoded using SHA256 hash and Base64 digest
    #[serde(rename = "expectedText")]
    pub expected_text: String,
    /// The start index(inclusive) for the redaction in specified content
    pub from: i64,
    /// The ending index(exclusive) for the redaction in specified content
    pub to: i64,
}
