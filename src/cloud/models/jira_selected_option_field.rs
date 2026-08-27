// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraSelectedOptionField {
    #[serde(rename = "optionId", default, skip_serializing_if = "Option::is_none")]
    pub option_id: Option<i64>,
}
