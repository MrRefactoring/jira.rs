// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The field configuration for an issue type.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConfigurationIssueTypeItem {
    /// The ID of the field configuration.
    #[serde(rename = "fieldConfigurationId")]
    pub field_configuration_id: String,
    /// The ID of the field configuration scheme.
    #[serde(rename = "fieldConfigurationSchemeId")]
    pub field_configuration_scheme_id: String,
    /// The ID of the issue type or *default*. When set to *default* this field configuration issue type item applies to all issue types without a field configuration.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
}
