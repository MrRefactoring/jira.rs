// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Invalid time date
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvalidSearchTimeDateError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Option<ApplicationError>>>,
}
