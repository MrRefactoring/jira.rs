// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraDateTimeInput {
    #[serde(rename = "formattedDateTime")]
    pub formatted_date_time: String,
}
