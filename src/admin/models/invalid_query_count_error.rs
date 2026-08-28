// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The number of queries exceeded the limit
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InvalidQueryCountError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Option<ApplicationError>>>,
}
