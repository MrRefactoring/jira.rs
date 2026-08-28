// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of custom field options for a context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CustomFieldUpdatedContextOptionsList {
    /// The updated custom field options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOptionUpdate>>,
}
