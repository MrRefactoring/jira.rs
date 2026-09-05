// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of scope.
    pub enum ScopeType {
        Project => "PROJECT",
        Template => "TEMPLATE",
    }
}

/// The projects the item is associated with. Indicated for items associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Scope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectDetails>,
    /// The type of scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ScopeType>,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for Scope {
    const FIELDS: &'static [&'static str] = &["project", "type"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
