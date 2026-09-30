// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details of new default levels.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetDefaultLevelsRequest {
    /// List of objects with issue security scheme ID and new default level ID.
    #[serde(rename = "defaultValues")]
    pub default_values: Vec<DefaultLevelValue>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for SetDefaultLevelsRequest {
    const FIELDS: &'static [&'static str] = &["defaultValues"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
