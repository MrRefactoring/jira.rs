// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The permission holder type. This must be "Group" or "AccountId".
    pub enum CreatePermissionHolderRequestType {
        Group => "Group",
        AccountId => "AccountId",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionHolderRequest {
    /// The permission holder type. This must be "Group" or "AccountId".
    pub r#type: CreatePermissionHolderRequestType,
    /// The permission holder value. This must be a group name if the type is "Group" or an account ID if the type is "AccountId".
    pub value: String,
}
