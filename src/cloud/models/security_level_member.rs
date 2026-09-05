// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue security level member.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SecurityLevelMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<PermissionHolder>,
    /// The ID of the issue security level member.
    pub id: String,
    /// The ID of the issue security level.
    #[serde(rename = "issueSecurityLevelId")]
    pub issue_security_level_id: String,
    /// The ID of the issue security scheme.
    #[serde(rename = "issueSecuritySchemeId")]
    pub issue_security_scheme_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for SecurityLevelMember {
    const FIELDS: &'static [&'static str] =
        &["holder", "id", "issueSecurityLevelId", "issueSecuritySchemeId", "managed"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
