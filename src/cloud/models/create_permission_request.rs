// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The permission type. This must be "View" or "Edit".
    pub enum CreatePermissionRequestType {
        View => "View",
        Edit => "Edit",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreatePermissionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub holder: Option<CreatePermissionHolderRequest>,
    /// The permission type. This must be "View" or "Edit".
    pub r#type: CreatePermissionRequestType,
}
