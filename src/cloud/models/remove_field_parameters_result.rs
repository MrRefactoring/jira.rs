// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Result of remove field parameters operation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RemoveFieldParametersResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SuccessOrErrorResults>>,
}
