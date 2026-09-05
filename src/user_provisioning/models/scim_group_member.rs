// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// SCIM group member
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ScimGroupMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}
