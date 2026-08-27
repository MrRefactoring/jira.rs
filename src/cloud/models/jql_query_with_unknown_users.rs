// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// JQL queries that contained users that could not be found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JQLQueryWithUnknownUsers {
    /// The converted query, with accountIDs instead of user identifiers, or 'unknown' for users that could not be found
    #[serde(rename = "convertedQuery", default, skip_serializing_if = "Option::is_none")]
    pub converted_query: Option<String>,
    /// The original query, for reference
    #[serde(rename = "originalQuery", default, skip_serializing_if = "Option::is_none")]
    pub original_query: Option<String>,
}
