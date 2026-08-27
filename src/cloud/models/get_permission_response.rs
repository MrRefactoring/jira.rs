// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The permission type. This is "View" or "Edit".
    pub enum GetPermissionResponseType {
        View => "View",
        Edit => "Edit",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<GetPermissionHolderResponse>,
    /// The permission type. This is "View" or "Edit".
    pub r#type: GetPermissionResponseType,
}
