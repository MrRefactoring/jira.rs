// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleRedactionRequest {
    #[serde(rename = "contentItem")]
    pub content_item: ContentItem,
    /// Unique id for the redaction request; ID format should be of UUID
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// The reason why the content is being redacted
    pub reason: String,
    #[serde(rename = "redactionPosition")]
    pub redaction_position: RedactionPosition,
}
