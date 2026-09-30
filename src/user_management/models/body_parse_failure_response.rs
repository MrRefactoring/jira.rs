// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The body was not parsed successfully.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BodyParseFailureResponse {
    pub key: String,
}
