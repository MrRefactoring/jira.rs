// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Allowed action for bulk edit shareable entity
    pub enum BulkEditShareableEntityResponseAction {
        ChangeOwner => "changeOwner",
        ChangePermission => "changePermission",
        AddPermission => "addPermission",
        RemovePermission => "removePermission",
    }
}

/// Details of a request to bulk edit shareable entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditShareableEntityResponse {
    /// Allowed action for bulk edit shareable entity
    pub action: BulkEditShareableEntityResponseAction,
    /// The mapping dashboard id to errors if any.
    #[serde(rename = "entityErrors", default, skip_serializing_if = "Option::is_none")]
    pub entity_errors: Option<std::collections::HashMap<String, serde_json::Value>>,
}
