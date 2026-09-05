// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Will treat as `MandatoryFieldValueForADF` if type is `adf`
    pub enum MandatoryFieldValueForADFType {
        Adf => "adf",
        Raw => "raw",
    }
}

/// An object notation input
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MandatoryFieldValueForADF {
    /// If `true`, will try to retain original non-null issue field values on move.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    /// Will treat as `MandatoryFieldValueForADF` if type is `adf`
    pub r#type: MandatoryFieldValueForADFType,
    /// Value for each field. Accepts Atlassian Document Format (ADF) for rich text fields like `description`, `environments`. For ADF format details, refer to: [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure)
    pub value: std::collections::HashMap<String, serde_json::Value>,
}
