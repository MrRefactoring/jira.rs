// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Result of updating JQL Function precomputations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JqlFunctionPrecomputationUpdateResponse {
    /// List of precomputations that were not found and skipped. Only returned if the request passed skipNotFoundPrecomputations=true.
    #[serde(rename = "notFoundPrecomputationIDs", default, skip_serializing_if = "Option::is_none")]
    pub not_found_precomputation_i_ds: Option<Vec<String>>,
}
