// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A reference to the location of the error. This will be null if the error does not refer to a specific element.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowElementReference {
    /// A property key.
    #[serde(rename = "propertyKey", default, skip_serializing_if = "Option::is_none")]
    pub property_key: Option<String>,
    /// A rule ID.
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "statusMappingReference", default, skip_serializing_if = "Option::is_none")]
    pub status_mapping_reference: Option<ProjectAndIssueTypePair>,
    /// A status reference.
    #[serde(rename = "statusReference", default, skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
    /// A transition ID.
    #[serde(rename = "transitionId", default, skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<String>,
}
