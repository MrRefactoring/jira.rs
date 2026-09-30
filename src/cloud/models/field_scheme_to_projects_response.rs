// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response for updating field scheme to projects associations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldSchemeToProjectsResponse {
    pub results: Vec<FieldSchemeToProjectsPartialFailure>,
}
