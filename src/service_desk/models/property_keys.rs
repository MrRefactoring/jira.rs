// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// List of property keys.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PropertyKeys {
    /// Property key details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<PropertyKey>>,
}
