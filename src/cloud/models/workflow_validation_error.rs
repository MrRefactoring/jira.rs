// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The validation error level.
    pub enum WorkflowValidationErrorLevel {
        Warning => "WARNING",
        Error => "ERROR",
    }
}

crate::open_enum! {
    /// The type of element the error or warning references.
    pub enum WorkflowValidationErrorType {
        Rule => "RULE",
        Status => "STATUS",
        StatusLayout => "STATUS_LAYOUT",
        StatusProperty => "STATUS_PROPERTY",
        Workflow => "WORKFLOW",
        Transition => "TRANSITION",
        TransitionProperty => "TRANSITION_PROPERTY",
        Scope => "SCOPE",
        StatusMapping => "STATUS_MAPPING",
        Trigger => "TRIGGER",
    }
}

/// The details about a workflow validation error.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowValidationError {
    /// Additional details about the validation error.
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
    /// An error code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "elementReference", default, skip_serializing_if = "Option::is_none")]
    pub element_reference: Option<WorkflowElementReference>,
    /// The validation error level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<WorkflowValidationErrorLevel>,
    /// An error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The type of element the error or warning references.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WorkflowValidationErrorType>,
}
