// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Issue security scheme and it's details
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateIssueSecuritySchemeDetails {
    /// The description of the issue security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The list of scheme levels which should be added to the security scheme.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<SecuritySchemeLevel>>,
    /// The name of the issue security scheme. Must be unique (case-insensitive).
    pub name: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for CreateIssueSecuritySchemeDetails {
    const FIELDS: &'static [&'static str] = &["description", "levels", "name"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
