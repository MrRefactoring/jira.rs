// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the operations that can be performed on the issue.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Operations {
    /// Details of the link groups defining issue operations.
    #[serde(rename = "linkGroups", default, skip_serializing_if = "Option::is_none")]
    pub link_groups: Option<Vec<LinkGroup>>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
