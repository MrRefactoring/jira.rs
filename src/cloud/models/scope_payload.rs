// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of the scope. Use `GLOBAL` or empty for company-managed project, and `PROJECT` for team-managed project
    pub enum ScopePayloadType {
        Global => "GLOBAL",
        Project => "PROJECT",
    }
}

/// The payload for creating a scope. Defines if a project is team-managed project or company-managed project
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScopePayload {
    /// The type of the scope. Use `GLOBAL` or empty for company-managed project, and `PROJECT` for team-managed project
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ScopePayloadType>,
}
