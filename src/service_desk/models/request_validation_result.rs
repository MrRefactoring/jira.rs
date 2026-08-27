// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestValidationResult {
    /// A single, human-readable summary describing why validation failed. Null when valid.
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// General validation errors that are not attributable to a single field. Empty when valid.
    #[serde(rename = "errorMessages", default, skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// Field-level validation errors, keyed by the failing request field id. Empty when valid.
    #[serde(rename = "fieldErrors", default, skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<Vec<RequestFieldValidationError>>,
    /// ProForma form validation errors, if a form was supplied. Empty when valid or no form was present.
    #[serde(rename = "formErrors", default, skip_serializing_if = "Option::is_none")]
    pub form_errors: Option<Vec<FormValidationError>>,
    /// A machine-readable reason key categorising the overall failure. Null when valid.
    #[serde(rename = "reasonKey", default, skip_serializing_if = "Option::is_none")]
    pub reason_key: Option<String>,
    /// True when the payload is both structurally and semantically valid and safe to create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
