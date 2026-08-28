// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Failure {
    /// Human readable error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Unique TraceId that can be used to find log messages.
    #[serde(rename = "traceId", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}
