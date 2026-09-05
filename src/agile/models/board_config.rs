// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BoardConfigRanking {
    #[serde(rename = "rankCustomFieldId", default, skip_serializing_if = "Option::is_none")]
    pub rank_custom_field_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BoardConfigSubQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BoardConfig {
    #[serde(rename = "columnConfig", default, skip_serializing_if = "Option::is_none")]
    pub column_config: Option<ColumnConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimation: Option<EstimationConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Relation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranking: Option<BoardConfigRanking>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "subQuery", default, skip_serializing_if = "Option::is_none")]
    pub sub_query: Option<BoardConfigSubQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
