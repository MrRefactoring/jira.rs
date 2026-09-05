// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkOperationResponse {
    pub errors: Vec<BulkTeamOperationError>,
    #[serde(rename = "successfulTeamIds")]
    pub successful_team_ids: Vec<String>,
}
