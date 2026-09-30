// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Details about an project using security scheme mapping.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IssueSecuritySchemeToProjectMapping {
    #[serde(rename = "issueSecuritySchemeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme_id: Option<String>,
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for IssueSecuritySchemeToProjectMapping {
    const FIELDS: &'static [&'static str] = &["issueSecuritySchemeId", "projectId"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
