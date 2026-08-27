// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServletOutputStream {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "writeListener", default, skip_serializing_if = "Option::is_none")]
    pub write_listener: Option<WriteListener>,
}
