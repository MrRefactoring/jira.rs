// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Response bean for field scheme parameter update operations.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateFieldSchemeParametersResponse {
    pub results: Vec<UpdateFieldSchemeParametersPartialFailure>,
}
