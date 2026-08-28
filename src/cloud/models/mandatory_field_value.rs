// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Will treat as `MandatoryFieldValue` if type is `raw` or `empty`
    pub enum MandatoryFieldValueType {
        Adf => "adf",
        Raw => "raw",
    }
}

/// List of string of inputs
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MandatoryFieldValue {
    /// If `true`, will try to retain original non-null issue field values on move.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// Will treat as `MandatoryFieldValue` if type is `raw` or `empty`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MandatoryFieldValueType>,
    /// Value for each field. Provide a `list of strings` for non-ADF fields.
    pub value: Vec<String>,
}
