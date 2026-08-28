// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The converted JQL queries.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConvertedJQLQueries {
    /// List of queries containing user information that could not be mapped to an existing user
    #[serde(rename = "queriesWithUnknownUsers", default, skip_serializing_if = "Option::is_none")]
    pub queries_with_unknown_users: Option<Vec<JQLQueryWithUnknownUsers>>,
    /// The list of converted query strings with account IDs in place of user identifiers.
    #[serde(rename = "queryStrings", default, skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<Vec<String>>,
}
