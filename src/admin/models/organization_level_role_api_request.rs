// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum OrganizationLevelRoleApiRequestRole {
        AtlassianOrgAdmin => "atlassian/org-admin",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationLevelRoleApiRequest {
    pub role: OrganizationLevelRoleApiRequestRole,
}
