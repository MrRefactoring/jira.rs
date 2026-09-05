// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachmentMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Upload limit in bytes
    #[serde(rename = "uploadLimit", default, skip_serializing_if = "Option::is_none")]
    pub upload_limit: Option<i64>,
}
