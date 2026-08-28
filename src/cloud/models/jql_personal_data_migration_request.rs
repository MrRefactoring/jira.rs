// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The JQL queries to be converted.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct JQLPersonalDataMigrationRequest {
    /// A list of queries with user identifiers. Maximum of 100 queries.
    #[serde(rename = "queryStrings", default, skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<Vec<String>>,
}
