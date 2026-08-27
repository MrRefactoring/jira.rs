// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormValidationErrorResponse {
    /// Description of the error.
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// A list of validation errors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FormValidationError>>,
    #[serde(rename = "i18nErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub i18n_error_message: Option<I18nErrorMessageDTO>,
}
