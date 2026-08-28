// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A list of changelog IDs.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueChangelogIds {
    /// The list of changelog IDs.
    #[serde(rename = "changelogIds")]
    pub changelog_ids: Vec<i64>,
}
