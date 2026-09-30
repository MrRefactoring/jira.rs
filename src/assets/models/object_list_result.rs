// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A result list containing objects
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectListResult {
    /// The actual objects
    #[serde(rename = "objectEntries")]
    pub object_entries: Vec<AssetObject>,
    /// The object type attributes that are present in the object entries
    #[serde(rename = "objectTypeAttributes", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attributes: Option<Vec<ObjectTypeAttribute>>,
    /// Deprecated field that shows which object type id the result is for. Not applicable when using AQL
    #[deprecated(note = "Deprecated field that shows which object type id the result is for.")]
    #[serde(rename = "objectTypeId", default, skip_serializing_if = "Option::is_none")]
    pub object_type_id: Option<String>,
    /// Deprecated field should not be used.
    #[deprecated(note = "Deprecated field should not be used.")]
    #[serde(rename = "objectTypeIsInherited", default, skip_serializing_if = "Option::is_none")]
    pub object_type_is_inherited: Option<bool>,
    /// Deprecated field should not be used.
    #[deprecated(note = "Deprecated field should not be used.")]
    #[serde(rename = "abstractObjectType", default, skip_serializing_if = "Option::is_none")]
    pub abstract_object_type: Option<bool>,
    /// The offset of the first object in the search query that is present in the result, used for pagination
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    /// The index of the last object present in the result of the search query
    #[serde(rename = "toIndex")]
    pub to_index: i64,
    /// The amount of objects currently returned per page in the result set
    #[serde(rename = "pageObjectSize")]
    pub page_object_size: i64,
    /// The current page of objects in the result set pagination
    #[serde(rename = "pageNumber")]
    pub page_number: i64,
    /// Deprecated field - The object type attribute id used for sorting
    #[deprecated(note = "Deprecated field - The object type attribute id used for sorting")]
    #[serde(rename = "orderByTypeAttrId", default, skip_serializing_if = "Option::is_none")]
    pub order_by_type_attr_id: Option<i64>,
    /// Deprecated field - The sort order, used in conjunction with the orderByTypeAttrId
    #[deprecated(note = "Deprecated field - The sort order, used in conjunction with the orderByTypeAttrId")]
    #[serde(rename = "orderWay", default, skip_serializing_if = "Option::is_none")]
    pub order_way: Option<String>,
    /// Deprecated field - The field is used for basic search
    #[deprecated(note = "Deprecated field - The field is used for basic search")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The AQL that was used to find the object result set
    #[serde(rename = "qlQuery")]
    pub ql_query: String,
    /// Determines if the query was based on an AQL or by basic search
    #[serde(rename = "qlQuerySearchResult", default, skip_serializing_if = "Option::is_none")]
    pub ql_query_search_result: Option<bool>,
    /// Is it possible to transform this AQL to basic search or vice versa
    #[serde(rename = "conversionPossible", default, skip_serializing_if = "Option::is_none")]
    pub conversion_possible: Option<bool>,
    /// Deprecated field should not be used
    #[deprecated(note = "Deprecated field should not be used")]
    #[serde(rename = "matchedFilterValues", default, skip_serializing_if = "Option::is_none")]
    pub matched_filter_values: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Deprecated field should not be used
    #[deprecated(note = "Deprecated field should not be used")]
    #[serde(rename = "inheritanceTree", default, skip_serializing_if = "Option::is_none")]
    pub inheritance_tree: Option<std::collections::HashMap<String, serde_json::Value>>,
}
