// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkWorklogKeyResponse {
    /// A list of successfully retrieved worklogs with their issue and worklog IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worklogs: Option<Vec<WorklogKeyResult>>,
}
