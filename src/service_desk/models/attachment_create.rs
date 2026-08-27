// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentCreate {
    #[serde(rename = "additionalComment", default, skip_serializing_if = "Option::is_none")]
    pub additional_comment: Option<AdditionalComment>,
    /// Controls whether the comment and its attachments are visible to customers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// List of IDs for the temporary attachments to be added to the customer request.
    #[serde(rename = "temporaryAttachmentIds", default, skip_serializing_if = "Option::is_none")]
    pub temporary_attachment_ids: Option<Vec<String>>,
}
