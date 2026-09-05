// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Bulk Edit Get Fields Response.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkEditGetFields {
    /// The end cursor for use in pagination.
    #[serde(rename = "endingBefore", default, skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// List of all the fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<IssueBulkEditField>>,
    /// The start cursor for use in pagination.
    #[serde(rename = "startingAfter", default, skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}
