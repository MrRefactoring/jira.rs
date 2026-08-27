// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The list of issue type IDs to be removed from the field configuration scheme.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueTypeIdsToRemove {
    /// The list of issue type IDs. Must contain unique values not longer than 255 characters and not be empty. Maximum of 100 IDs.
    #[serde(rename = "issueTypeIds")]
    pub issue_type_ids: Vec<String>,
}
