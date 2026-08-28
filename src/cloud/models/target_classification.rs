// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Classification mapping for classifications in source issues to respective target classification.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TargetClassification {
    /// An object with the key as the ID of the target classification and value with the list of the IDs of the current source classifications.
    pub classifications: std::collections::HashMap<String, serde_json::Value>,
    /// ID of the source issueType to which issues present in `issueIdOrKeys` belongs.
    #[serde(rename = "issueType", default, skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    /// ID or key of the source project to which issues present in `issueIdOrKeys` belongs.
    #[serde(rename = "projectKeyOrId", default, skip_serializing_if = "Option::is_none")]
    pub project_key_or_id: Option<String>,
}
