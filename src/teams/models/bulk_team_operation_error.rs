// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkTeamOperationError {
    pub code: String,
    pub message: String,
    #[serde(rename = "teamId")]
    pub team_id: String,
}
