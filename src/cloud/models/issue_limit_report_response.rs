// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueLimitReportResponse {
    /// For each field, the ids of the individual entities breaching the limit, grouped by the id or key of the issue they belong to. Fields that hold a single value, such as description and environment, map to an empty list because the issue itself identifies the breaching content
    #[serde(rename = "entitiesBreachingLimit", default, skip_serializing_if = "Option::is_none")]
    pub entities_breaching_limit: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// A list of ids of issues approaching the limit and their field count
    #[serde(rename = "issuesApproachingLimit", default, skip_serializing_if = "Option::is_none")]
    pub issues_approaching_limit: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// A list of ids of issues breaching the limit and their field count
    #[serde(rename = "issuesBreachingLimit", default, skip_serializing_if = "Option::is_none")]
    pub issues_breaching_limit: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The fields and their defined limits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<std::collections::HashMap<String, serde_json::Value>>,
}
