// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The payload for defining quick filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuickFilterPayload {
    /// The description of the quick filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The jql query for the quick filter
    #[serde(rename = "jqlQuery", default, skip_serializing_if = "Option::is_none")]
    pub jql_query: Option<String>,
    /// The name of the quick filter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
