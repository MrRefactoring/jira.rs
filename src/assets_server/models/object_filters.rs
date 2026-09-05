// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ObjectFilters {
    #[serde(rename = "objectSchemaId", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_id: Option<i64>,
    #[serde(rename = "qlQuerySearch", default, skip_serializing_if = "Option::is_none")]
    pub ql_query_search: Option<bool>,
    #[serde(rename = "qlQueryParams", default, skip_serializing_if = "Option::is_none")]
    pub ql_query_params: Option<ObjectIQLFilterParam>,
    #[serde(rename = "filterParams", default, skip_serializing_if = "Option::is_none")]
    pub filter_params: Option<ObjectFilterParams>,
    #[serde(rename = "iqlSearch", default, skip_serializing_if = "Option::is_none")]
    pub iql_search: Option<bool>,
    #[serde(rename = "iqlParams", default, skip_serializing_if = "Option::is_none")]
    pub iql_params: Option<ObjectIQLFilterParam>,
}
