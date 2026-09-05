// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Minimal response for updating field scheme to fields associations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MinimalFieldSchemeToFieldsResponse {
    pub results: Vec<MinimalFieldSchemeToFieldsPartialFailure>,
}
