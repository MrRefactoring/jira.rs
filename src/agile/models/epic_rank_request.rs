// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EpicRankRequest {
    #[serde(rename = "rankAfterEpic", default, skip_serializing_if = "Option::is_none")]
    pub rank_after_epic: Option<String>,
    #[serde(rename = "rankBeforeEpic", default, skip_serializing_if = "Option::is_none")]
    pub rank_before_epic: Option<String>,
    #[serde(rename = "rankCustomFieldId", default, skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}
