// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Update projects in a scheme
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateProjectsInSchemeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<PrioritySchemeChangesWithoutMappings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<PrioritySchemeChangesWithoutMappings>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for UpdateProjectsInSchemeRequest {
    const FIELDS: &'static [&'static str] = &["add", "remove"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
