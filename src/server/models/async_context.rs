// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AsyncContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Box<ServletRequest>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<ServletResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
