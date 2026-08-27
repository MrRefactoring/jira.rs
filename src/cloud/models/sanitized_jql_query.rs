// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of the sanitized JQL query.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SanitizedJqlQuery {
    /// The account ID of the user for whom sanitization was performed.
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<ErrorCollection>,
    /// The initial query.
    #[serde(rename = "initialQuery", default, skip_serializing_if = "Option::is_none")]
    pub initial_query: Option<String>,
    /// The sanitized query, if there were no errors.
    #[serde(rename = "sanitizedQuery", default, skip_serializing_if = "Option::is_none")]
    pub sanitized_query: Option<String>,
}
