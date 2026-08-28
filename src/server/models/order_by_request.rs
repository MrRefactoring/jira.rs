// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderByRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jql: Option<String>,
    #[serde(rename = "maxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
