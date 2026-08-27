// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The type of the transition.
    pub enum TransitionType {
        Global => "global",
        Initial => "initial",
        Directed => "directed",
    }
}

/// Details of a workflow transition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
    /// The description of the transition.
    pub description: String,
    /// The statuses the transition can start from.
    pub from: Vec<String>,
    /// The ID of the transition.
    pub id: String,
    /// The name of the transition.
    pub name: String,
    /// The properties of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<WorkflowRules>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub screen: Option<TransitionScreenDetails>,
    /// The status the transition goes to.
    pub to: String,
    /// The type of the transition.
    pub r#type: TransitionType,
}
