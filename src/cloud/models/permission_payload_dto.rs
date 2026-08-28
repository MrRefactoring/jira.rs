// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The strategy to use when there is a conflict with an existing permission scheme. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - If the entity exist, try and create a new one with a different name
    pub enum PermissionPayloadDTOOnConflict {
        Fail => "FAIL",
        Use => "USE",
        New => "NEW",
    }
}

/// The payload to create a permission scheme
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PermissionPayloadDTO {
    /// Configuration to generate addon role. Default is false if null. Only applies to GLOBAL-scoped permission scheme
    #[serde(rename = "addAddonRole", default, skip_serializing_if = "Option::is_none")]
    pub add_addon_role: Option<bool>,
    /// The description of the permission scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of permission grants
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<PermissionGrantDTO>>,
    /// The name of the permission scheme
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The strategy to use when there is a conflict with an existing permission scheme. FAIL - Fail execution, this always needs to be unique; USE - Use the existing entity and ignore new entity parameters; NEW - If the entity exist, try and create a new one with a different name
    #[serde(rename = "onConflict", default, skip_serializing_if = "Option::is_none")]
    pub on_conflict: Option<PermissionPayloadDTOOnConflict>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pcri: Option<ProjectCreateResourceIdentifier>,
}
