// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Describes who/how the team is managed. The possible values are
    /// \* external - when team is synced from an external directory like SCIM or HRIS, and team members cannot be modified.
    /// \* admins - when a team is managed by an admin (team members can only be modified by admins).
    /// \* team-members - managed by existing team members, new members need to be invited to join.
    /// \* open - anyone can join or modify this team.
    pub enum FoundGroupManagedBy {
        External => "EXTERNAL",
        Admins => "ADMINS",
        TeamMembers => "TEAM_MEMBERS",
        Open => "OPEN",
    }
}

crate::open_enum! {
    /// Describes the type of group. The possible values are
    /// \* team-collaboration - A platform team managed in people directory.
    /// \* userbase-group - a group of users created in adminhub.
    /// \* admin-oversight - currently unused.
    pub enum FoundGroupUsageType {
        UserbaseGroup => "USERBASE_GROUP",
        TeamCollaboration => "TEAM_COLLABORATION",
        AdminOversight => "ADMIN_OVERSIGHT",
    }
}

/// A group found in a search.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundGroup {
    /// Avatar url for the group/team if present.
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The ID of the group, which uniquely identifies the group across all Atlassian products. For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*.
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The group name with the matched query string highlighted with the HTML bold tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<GroupLabel>>,
    /// Describes who/how the team is managed. The possible values are
    /// \* external - when team is synced from an external directory like SCIM or HRIS, and team members cannot be modified.
    /// \* admins - when a team is managed by an admin (team members can only be modified by admins).
    /// \* team-members - managed by existing team members, new members need to be invited to join.
    /// \* open - anyone can join or modify this team.
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<FoundGroupManagedBy>,
    /// The name of the group. The name of a group is mutable, to reliably identify a group use ``groupId`.`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Describes the type of group. The possible values are
    /// \* team-collaboration - A platform team managed in people directory.
    /// \* userbase-group - a group of users created in adminhub.
    /// \* admin-oversight - currently unused.
    #[serde(rename = "usageType", default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<FoundGroupUsageType>,
}
