// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCreateValidateRequest {
    pub payload: WorkflowCreateRequest,
    #[serde(rename = "validationOptions", default, skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<ValidationOptionsForCreate>,
}
