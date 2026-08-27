// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of field configuration items.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldConfigurationItemsDetails {
    /// Details of fields in a field configuration.
    #[serde(rename = "fieldConfigurationItems")]
    pub field_configuration_items: Vec<FieldConfigurationItem>,
}
