// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomerRequestCreateMeta {
    #[serde(rename = "requestTypeFields", default, skip_serializing_if = "Option::is_none")]
    pub request_type_fields: Option<Vec<RequestTypeField>>,
    #[serde(rename = "canRaiseOnBehalfOf", default, skip_serializing_if = "Option::is_none")]
    pub can_raise_on_behalf_of: Option<bool>,
    #[serde(rename = "canAddRequestParticipants", default, skip_serializing_if = "Option::is_none")]
    pub can_add_request_participants: Option<bool>,
}
