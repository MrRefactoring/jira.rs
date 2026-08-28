// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ValidationOptionsForCreateLevels {
        Warning => "WARNING",
        Error => "ERROR",
    }
}

/// The level of validation to return from the API. If no values are provided, the default would return `WARNING` and `ERROR` level validation results.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ValidationOptionsForCreate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<ValidationOptionsForCreateLevels>>,
}
