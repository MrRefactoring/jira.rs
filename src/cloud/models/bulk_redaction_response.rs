// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BulkRedactionResponse {
    /// Result for requested redactions
    pub results: Vec<SingleRedactionResponse>,
}
