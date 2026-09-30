// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkWorklogKeyRequest {
    /// A list of issue and worklog ID pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<Vec<WorklogCompositeKey>>,
}
