// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerRequestActions {
    #[serde(rename = "addAttachment", default, skip_serializing_if = "Option::is_none")]
    pub add_attachment: Option<CustomerRequestAction>,
    #[serde(rename = "addComment", default, skip_serializing_if = "Option::is_none")]
    pub add_comment: Option<CustomerRequestAction>,
    #[serde(rename = "addParticipant", default, skip_serializing_if = "Option::is_none")]
    pub add_participant: Option<CustomerRequestAction>,
    #[serde(rename = "removeParticipant", default, skip_serializing_if = "Option::is_none")]
    pub remove_participant: Option<CustomerRequestAction>,
}
