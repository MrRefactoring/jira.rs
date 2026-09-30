// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A filter object that is used to find a paginated result set based on an object type and an AQL query
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ObjectFilterParams {
    /// The AQL that will fetch the objects. The object type parameter will be appended implicitly to this AQL
    #[serde(rename = "qlQuery")]
    pub ql_query: String,
    #[serde(rename = "objectTypeId")]
    pub object_type_id: String,
    /// The requested page to be loaded for a paginated result. The default value is page = 1
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// How many objects should be returned in the request. It is used with page attribute for pagination.
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: i64,
    /// Which attribute should be used to order by. The preferred way is to use an order by in `qlQuery` and not pass this argument.
    #[serde(rename = "orderByTypeAttrId", default, skip_serializing_if = "Option::is_none")]
    pub order_by_type_attr_id: Option<i64>,
    /// Sort objects in ascending order or descending order based on the attribute identified by orderByTypeAttrId. 1 means ascending all other values mean descending. The preferred way is to not supply the asc parameter and use an order by in `qlQuery` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asc: Option<i64>,
    /// Identifies an object that should be included in the result. The page will be calculated accordingly to include the object specified in the result set
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "objectSchemaId")]
    pub object_schema_id: String,
    /// Should attribute values be included in the response.
    #[serde(rename = "includeAttributes", default, skip_serializing_if = "Option::is_none")]
    pub include_attributes: Option<bool>,
    /// Identifies the attributes which values should be included in the response. Note that the includeAttributes must be specified to true in order for this parameter to be used.
    #[serde(rename = "attributesToDisplay", default, skip_serializing_if = "Option::is_none")]
    pub attributes_to_display: Option<ObjectTypeAttributesToDisplay>,
}
