// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Error response returned updating JQL Function precomputations fails.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlFunctionPrecomputationUpdateErrorResponse {
    /// The list of error messages produced by this operation.
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// List of precomputations that were not found.
    #[serde(rename = "notFoundPrecomputationIDs", default, skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_i_ds: Option<Vec<String>>,
}
