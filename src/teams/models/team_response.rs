// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum TeamResponseState {
        Active => "ACTIVE",
        Archived => "ARCHIVED",
    }
}

crate::open_enum! {
    pub enum TeamResponseTeamType {
        Open => "OPEN",
        MemberInvite => "MEMBER_INVITE",
        External => "EXTERNAL",
        OrgAdminManaged => "ORG_ADMIN_MANAGED",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TeamResponse {
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    pub description: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "externalReference", default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<TeamExternalReference>,
    #[serde(rename = "organizationId")]
    pub organization_id: String,
    pub state: TeamResponseState,
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "teamType")]
    pub team_type: TeamResponseTeamType,
    #[serde(rename = "userPermissions")]
    pub user_permissions: UserPermissions,
}
