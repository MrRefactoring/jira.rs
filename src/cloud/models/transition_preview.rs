// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The transition type.
    pub enum TransitionPreviewType {
        Initial => "INITIAL",
        Global => "GLOBAL",
        Directed => "DIRECTED",
    }
}

/// Details about a workflow transition in preview context.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransitionPreview {
    /// The post-functions of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<PreviewRuleConfiguration>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Box<PreviewConditionGroupConfiguration>>,
    /// The custom issue event ID for the transition.
    #[serde(rename = "customIssueEventId", default, skip_serializing_if = "Option::is_none")]
    pub custom_issue_event_id: Option<String>,
    /// The description of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The statuses the transition can start from, and the mapping of ports between the statuses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TransitionLink>>,
    /// The name of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status the transition goes to.
    #[serde(rename = "toStatusReference", default, skip_serializing_if = "Option::is_none")]
    pub to_status_reference: Option<String>,
    #[serde(rename = "transitionScreen", default, skip_serializing_if = "Option::is_none")]
    pub transition_screen: Option<PreviewRuleConfiguration>,
    /// The triggers of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<PreviewTrigger>>,
    /// The transition type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TransitionPreviewType>,
    /// The validators of the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<PreviewRuleConfiguration>>,
}
