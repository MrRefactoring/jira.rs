// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Group detail information.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryGroupDetailsData {
    /// Unique ID of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The group name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The group description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the directory.
    #[serde(rename = "directoryId", default, skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "managementAccess", default, skip_serializing_if = "Option::is_none")]
    pub management_access: Option<ManagementAccess>,
    /// Indication if group was created via IdP Sync.
    #[serde(rename = "externalSynced", default, skip_serializing_if = "Option::is_none")]
    pub external_synced: Option<bool>,
    /// Specifies how the group is managed: external, admins, team-members, or open.
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<GroupCounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkSelfCursor>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryGroupDetails {
    /// Group detail information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<MultiDirectoryGroupDetailsData>,
}
