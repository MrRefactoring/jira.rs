// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The list of issue type IDs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeIds {
    /// The list of issue type IDs.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
}
