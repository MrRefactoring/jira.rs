// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SaveTemplateResponse {
    #[serde(rename = "projectTemplateKey", default, skip_serializing_if = "Option::is_none")]
    pub project_template_key: Option<ProjectTemplateKey>,
}
