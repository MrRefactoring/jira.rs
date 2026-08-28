// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of configurations for a custom field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldConfigurations {
    /// The list of custom field configuration details.
    pub configurations: Vec<ContextualConfiguration>,
}
