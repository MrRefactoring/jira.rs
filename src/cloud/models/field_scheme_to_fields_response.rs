// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response for updating field associations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldSchemeToFieldsResponse {
    pub results: Vec<FieldSchemeToFieldsPartialFailure>,
}
