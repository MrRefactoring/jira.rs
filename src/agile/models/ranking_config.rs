// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RankingConfig {
    #[serde(rename = "rankCustomFieldId", default, skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}
