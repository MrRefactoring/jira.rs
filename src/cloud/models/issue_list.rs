// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A list of issue IDs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueList {
    /// The list of issue IDs.
    #[serde(rename = "issueIds")]
    pub issue_ids: Vec<String>,
}
