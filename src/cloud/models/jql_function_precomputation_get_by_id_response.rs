// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Get precomputations by ID response.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JqlFunctionPrecomputationGetByIdResponse {
    /// List of precomputations that were not found.
    #[serde(rename = "notFoundPrecomputationIDs", default, skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_i_ds: Option<Vec<String>>,
    /// The list of precomputations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precomputations: Option<Vec<JqlFunctionPrecomputation>>,
}
