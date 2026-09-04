// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A list of editable field details.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueUpdateMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
}
