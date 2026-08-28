// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OperationMessage {
    /// The human-readable message that describes the result.
    pub message: String,
    /// The status code of the response.
    #[serde(rename = "statusCode")]
    pub status_code: i64,
}
