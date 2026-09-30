// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Result for requested redactions
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SingleRedactionResponse {
    /// An unique id for the redaction request
    #[serde(rename = "externalId")]
    pub external_id: String,
    /// Indicates if redaction was success/failure
    pub successful: bool,
}
