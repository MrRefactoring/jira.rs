// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The statuses associated with this workflow.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusLayoutUpdate {
    #[serde(rename = "approvalConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub approval_configuration: Option<ApprovalConfiguration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<WorkflowLayout>,
    /// The properties for this status layout.
    pub properties: std::collections::HashMap<String, serde_json::Value>,
    /// A unique ID which the status will use to refer to this layout configuration.
    #[serde(rename = "statusReference")]
    pub status_reference: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}

impl crate::core::Extensible for StatusLayoutUpdate {
    const FIELDS: &'static [&'static str] = &["approvalConfiguration", "layout", "properties", "statusReference"];

    fn additional(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.additional
    }

    fn additional_mut(&mut self) -> &mut std::collections::HashMap<String, serde_json::Value> {
        &mut self.additional
    }
}
