// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Internal error
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InternalError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Option<ApplicationError>>>,
}
