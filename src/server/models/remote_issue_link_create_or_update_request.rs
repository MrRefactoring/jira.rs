// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoteIssueLinkCreateOrUpdateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Application>,
    #[serde(rename = "globalId", default, skip_serializing_if = "Option::is_none")]
    pub global_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<RemoteObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}
