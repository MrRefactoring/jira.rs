// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SearchWorkspacesOperand {
    /// Returns workspaces, which partially contain the specified text in workspace name or url.
    #[serde(rename = "searchWorkspaces", default, skip_serializing_if = "Option::is_none")]
    pub search_workspaces: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
