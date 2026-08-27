// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraGroupInput {
    #[serde(rename = "groupName")]
    pub group_name: String,
}
