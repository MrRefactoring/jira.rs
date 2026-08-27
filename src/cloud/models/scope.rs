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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
