// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectListResult {
    #[serde(rename = "objectEntries", default, skip_serializing_if = "Option::is_none")]
    pub object_entries: Option<Vec<AssetObject>>,
    #[serde(rename = "objectTypeAttributes", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attributes: Option<Vec<ObjectTypeAttribute>>,
    #[serde(rename = "objectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub object_type_id: Option<i64>,
    #[serde(rename = "objectTypeIsInherited", default, skip_serializing_if = "Option::is_none")]
    pub object_type_is_inherited: Option<bool>,
    #[serde(rename = "abstractObjectType", default, skip_serializing_if = "Option::is_none")]
    pub abstract_object_type: Option<bool>,
    #[serde(rename = "totalFilterCount", default, skip_serializing_if = "Option::is_none")]
    pub total_filter_count: Option<i64>,
    #[serde(rename = "startIndex", default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(rename = "toIndex", default, skip_serializing_if = "Option::is_none")]
    pub to_index: Option<i64>,
    #[serde(rename = "pageObjectSize", default, skip_serializing_if = "Option::is_none")]
    pub page_object_size: Option<i64>,
    #[serde(rename = "pageNumber", default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    #[serde(rename = "orderByTypeAttrId", default, skip_serializing_if = "Option::is_none")]
    pub order_by_type_attr_id: Option<i64>,
    #[serde(rename = "orderWay", default, skip_serializing_if = "Option::is_none")]
    pub order_way: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ObjectFilterValues>>,
    #[serde(rename = "qlQuery", default, skip_serializing_if = "Option::is_none")]
    pub ql_query: Option<String>,
    #[serde(rename = "qlQuerySearchResult", default, skip_serializing_if = "Option::is_none")]
    pub ql_query_search_result: Option<bool>,
    #[serde(rename = "conversionPossible", default, skip_serializing_if = "Option::is_none")]
    pub conversion_possible: Option<bool>,
    #[serde(rename = "matchedFilterValues", default, skip_serializing_if = "Option::is_none")]
    pub matched_filter_values: Option<Vec<ObjectAttribute>>,
    #[serde(rename = "inheritanceTree", default, skip_serializing_if = "Option::is_none")]
    pub inheritance_tree: Option<ObjectTypeInheritanceTree>,
    #[serde(rename = "orderAscending", default, skip_serializing_if = "Option::is_none")]
    pub order_ascending: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iql: Option<String>,
    #[serde(rename = "iqlSearchResult", default, skip_serializing_if = "Option::is_none")]
    pub iql_search_result: Option<bool>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
