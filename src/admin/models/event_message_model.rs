// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventMessageModel {
    /// Encrypted message of audit log activity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Format of the audit log message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}
