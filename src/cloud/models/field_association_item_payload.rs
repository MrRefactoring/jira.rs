// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Defines the payload for the field association scheme.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FieldAssociationItemPayload {
    /// The description of the field association item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
    #[serde(rename = "qualifierId", default, skip_serializing_if = "Option::is_none")]
    pub qualifier_id: Option<ProjectCreateResourceIdentifier>,
    #[serde(rename = "qualifierType", default, skip_serializing_if = "Option::is_none")]
    pub qualifier_type: Option<ProjectCreateResourceIdentifier>,
    /// The renderer type of the field
    #[serde(rename = "rendererType", default, skip_serializing_if = "Option::is_none")]
    pub renderer_type: Option<String>,
    /// Whether the field is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
