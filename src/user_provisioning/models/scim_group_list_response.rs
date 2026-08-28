// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// SCIM group list response
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScimGroupListResponse {
    /// SCIM schemas that define list of response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    /// The total number of results returned by the query operation. The value may be larger than  the number of resources returned, such as when returning a single page of results from a  larger result set.
    #[serde(rename = "totalResults", default, skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i64>,
    /// The 1-based index of the first result in the current set of list results.
    #[serde(rename = "startIndex", default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    /// The number of resources returned in a list response page.
    #[serde(rename = "itemsPerPage", default, skip_serializing_if = "Option::is_none")]
    pub items_per_page: Option<i64>,
    /// The list of resource objects.
    #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ScimGroup>>,
}
