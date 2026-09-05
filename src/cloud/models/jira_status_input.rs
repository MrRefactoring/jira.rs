// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraStatusInput {
    #[serde(rename = "statusId")]
    pub status_id: String,
}
