// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MultidirectoryInviteRoleAssociation {
    /// A resource or workspace refers to a specific instance of an Atlassian app, which has a unique ID. Use the [Get Workspaces endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-workspaces/#api-v2-orgs-orgid-workspaces-post) to find the resource ID.
    pub resource: String,
    /// Role to assign to a resource.
    /// Valid values:
    ///   - `atlassian/user`
    ///   - `atlassian/admin`
    ///   - `atlassian/guest`
    ///   - `atlassian/contributor`
    ///   - `atlassian/customer`
    ///   - `atlassian/basic`
    ///   - `atlassian/stakeholder`
    ///   - `atlassian/viewer`
    pub role: String,
}
