// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetGroupRoleAssignmentsRequestRoleIds {
        AtlassianUser => "atlassian/user",
        AtlassianAdmin => "atlassian/admin",
        AtlassianGuest => "atlassian/guest",
        AtlassianCustomer => "atlassian/customer",
        AtlassianUserAccessAdmin => "atlassian/user-access-admin",
        AtlassianContributor => "atlassian/contributor",
        AtlassianBasic => "atlassian/basic",
        AtlassianStakeholder => "atlassian/stakeholder",
        AtlassianOrgAdmin => "atlassian/org-admin",
        AtlassianSiteAdmin => "atlassian/site-admin",
        AtlassianAiAccess => "atlassian/ai-access",
    }
}

crate::open_enum! {
    pub enum GetGroupsCountRequestRoleIds {
        AtlassianUser => "atlassian/user",
        AtlassianAdmin => "atlassian/admin",
        AtlassianGuest => "atlassian/guest",
        AtlassianCustomer => "atlassian/customer",
        AtlassianUserAccessAdmin => "atlassian/user-access-admin",
        AtlassianContributor => "atlassian/contributor",
        AtlassianBasic => "atlassian/basic",
        AtlassianStakeholder => "atlassian/stakeholder",
        AtlassianOrgAdmin => "atlassian/org-admin",
        AtlassianSiteAdmin => "atlassian/site-admin",
        AtlassianAiAccess => "atlassian/ai-access",
    }
}

/// Whether to include counts of different objects associated with the group.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetGroupsRequestCounts {
    /// Whether to include the number of resources associated with the group.
    #[serde(rename = "includeResources", default, skip_serializing_if = "Option::is_none")]
    pub include_resources: Option<bool>,
    /// Whether to include the number of users associated with the group.
    #[serde(rename = "includeUsers", default, skip_serializing_if = "Option::is_none")]
    pub include_users: Option<bool>,
}

crate::open_enum! {
    /// The name of the field to sort the results by.
    pub enum GetGroupsRequestSortByField {
        Name => "name",
    }
}

crate::open_enum! {
    /// The direction to sort the results by.
    pub enum GetGroupsRequestSortByDirection {
        Asc => "asc",
        Desc => "desc",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupsRequestSortBy {
    /// The name of the field to sort the results by.
    pub field: GetGroupsRequestSortByField,
    /// The direction to sort the results by.
    pub direction: GetGroupsRequestSortByDirection,
}

crate::open_enum! {
    pub enum GetGroupsRequestRoleIds {
        AtlassianUser => "atlassian/user",
        AtlassianAdmin => "atlassian/admin",
        AtlassianGuest => "atlassian/guest",
        AtlassianCustomer => "atlassian/customer",
        AtlassianUserAccessAdmin => "atlassian/user-access-admin",
        AtlassianContributor => "atlassian/contributor",
        AtlassianBasic => "atlassian/basic",
        AtlassianStakeholder => "atlassian/stakeholder",
        AtlassianOrgAdmin => "atlassian/org-admin",
        AtlassianSiteAdmin => "atlassian/site-admin",
        AtlassianAiAccess => "atlassian/ai-access",
    }
}

/// The Groups operations.
pub struct GroupsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GroupsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Return a page of groups in an organization that match the supplied parameters.
    ///
    /// Use `searchTerm` for free-text search across group names. Filter by IDs, role assignments, resources, members, or specific group identifiers using the corresponding request fields. Use the `expand` field to include additional fields such as `counts.resources` and `counts.users` in the response.
    pub fn search_directory_groups(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
    ) -> SearchDirectoryGroupsRequest<'a> {
        SearchDirectoryGroupsRequest::new(self.client, org_id, directory_id)
    }

    /// Returns a page of role assignments for a group that match the supplied parameters.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:groups:admin`
    pub fn get_group_role_assignments(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> GetGroupRoleAssignmentsRequest<'a> {
        GetGroupRoleAssignmentsRequest::new(self.client, org_id, directory_id, group_id)
    }

    /// Assign a role to a group to assign all members the same role.
    pub fn grant_group_access(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        resource_id: impl Into<String>,
        role_id: impl Into<String>,
    ) -> GrantGroupAccessRequest<'a> {
        GrantGroupAccessRequest::new(self.client, org_id, directory_id, group_id, resource_id, role_id)
    }

    /// Revoke a role from a group to remove access to an app from all members. A member can still access the app if they’re in another group that grants access to the same app.
    pub fn revoke_group_access(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        resource_id: impl Into<String>,
        role_id: impl Into<String>,
    ) -> RevokeGroupAccessRequest<'a> {
        RevokeGroupAccessRequest::new(self.client, org_id, directory_id, group_id, resource_id, role_id)
    }

    /// Add a user to a group. This gives the user the same app access and permissions as the group. The user must be in the same directory as the group.
    ///
    /// **Note:** Adding a user to the org-admin group through this API will return an error after the Units rollout. The org-admin group will no longer grant organization admin access after the rollout. To grant organization admin, use the [Assign organization-level role endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/#api-v1-orgs-orgid-users-userid-role-assignments-assign-post) instead. This applies to all organizations, not just unit organizations.
    ///
    /// You can’t add a user to a group synced from an identity provider. Manage this group in your identity provider instead.
    ///
    /// You can’t add a user to a group if you’ve exceeded your user limit for an app that the group grants access to. Increase your user limit or suspend another user from the app first.
    pub fn add_user_to_group(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> AddUserToGroupRequest<'a> {
        AddUserToGroupRequest::new(self.client, org_id, directory_id, group_id, account_id)
    }

    /// Remove a user from a group. This removes any app access and permissions granted by this group, but the user may still be in other groups that grant the same app access and permissions.
    pub fn remove_user_from_group(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> RemoveUserFromGroupRequest<'a> {
        RemoveUserFromGroupRequest::new(self.client, org_id, directory_id, group_id, account_id)
    }

    /// Returns the details of a group.
    pub fn get_group(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> GetGroupRequest<'a> {
        GetGroupRequest::new(self.client, org_id, directory_id, group_id)
    }

    /// Delete a group from a directory if you don’t need this group anymore. This removes any app access and permissions granted by this group from all members. A member can still access an app if they’re in another group that grants access to the same app.
    pub fn delete_group(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> DeleteGroupRequest<'a> {
        DeleteGroupRequest::new(self.client, org_id, directory_id, group_id)
    }

    /// Returns the count of groups in an organization that match the supplied parameters.
    pub fn get_groups_count(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
    ) -> GetGroupsCountRequest<'a> {
        GetGroupsCountRequest::new(self.client, org_id, directory_id)
    }

    /// Returns group stats for the organization.
    pub fn get_groups_stats(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
    ) -> GetGroupsStatsRequest<'a> {
        GetGroupsStatsRequest::new(self.client, org_id, directory_id)
    }

    /// **This API is deprecated and will no longer work after June 30, 2027.** Use the [Search for groups in an organization endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/#api-v2-orgs-orgid-directories-directoryid-groups-search-post) instead.
    ///
    /// Returns a page of groups in an organization that match the supplied parameters.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:groups:admin`
    #[deprecated(
        note = "**This API is deprecated and will no longer work after June 30, 2027.** Use the [Search for groups in an organization endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/#api-v2-orgs-orgid-directories-directoryid-groups-search-post) instead."
    )]
    pub fn get_groups(&self, org_id: impl Into<String>, directory_id: impl Into<String>) -> GetGroupsRequest<'a> {
        GetGroupsRequest::new(self.client, org_id, directory_id)
    }

    /// Create a group in a directory to manage app access and permissions for multiple users together.
    pub fn create_group(
        &self,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        name: impl Into<String>,
    ) -> CreateGroupRequest<'a> {
        CreateGroupRequest::new(self.client, org_id, directory_id, name)
    }
}

/// Return a page of groups in an organization that match the supplied parameters.
///
/// Use `searchTerm` for free-text search across group names. Filter by IDs, role assignments, resources, members, or specific group identifiers using the corresponding request fields. Use the `expand` field to include additional fields such as `counts.resources` and `counts.users` in the response.
pub struct SearchDirectoryGroupsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    multi_directory_group_search_request: Option<MultiDirectoryGroupSearchRequest>,
}

impl<'a> SearchDirectoryGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, directory_id: impl Into<String>) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            multi_directory_group_search_request: None,
        }
    }

    #[must_use]
    pub fn multi_directory_group_search_request(mut self, value: MultiDirectoryGroupSearchRequest) -> Self {
        self.multi_directory_group_search_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/search",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id)
            ),
        );

        let body = match serde_json::to_value(&self.multi_directory_group_search_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MultiDirectoryGroupSearchPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a page of role assignments for a group that match the supplied parameters.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:groups:admin`
pub struct GetGroupRoleAssignmentsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
    cursor: Option<String>,
    limit: Option<i64>,
    directory_ids: Option<Vec<String>>,
    resource_owners: Option<Vec<String>>,
    resource_ids: Option<Vec<String>>,
    role_ids: Option<Vec<GetGroupRoleAssignmentsRequestRoleIds>>,
}

impl<'a> GetGroupRoleAssignmentsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            group_id: group_id.into(),
            cursor: None,
            limit: None,
            directory_ids: None,
            resource_owners: None,
            resource_ids: None,
            role_ids: None,
        }
    }

    /// Sets the cursor position to retrieve the next set of results. If present, all other parameters are discarded when searching.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// The desired number of results for the search request.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// A list of directory IDs. The requestor must have permissions to  administer resources linked to these directories.
    #[must_use]
    pub fn directory_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.directory_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of resource owners to filter the results by. Used to identify resources using their owner to which the user has at least one role assigned to.
    #[must_use]
    pub fn resource_owners(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_owners = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of resource IDs. The resource IDs should be specified  using the Atlassian Resource Identifier (ARI) format. Example ARI: `ari:cloud:jira-core::site/1`
    #[must_use]
    pub fn resource_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of role IDs. The Atlassian canonical roles are used to determine the permissions of the user against resources within  the organization. The allowed roles are:
    ///    - `atlassian/user` - Can access the product, with no product admin permissions
    ///    - `atlassian/admin` - Can access the product, with product admin permissions
    ///    - `atlassian/guest` - Can only access one space you or space admins specify
    ///    - `atlassian/customer` - (Jira Service Management) Can visit help center, submit help requests, and view articles (non-billable)
    ///    - `atlassian/user-access-admin` - No product access. Can administer users and groups for this product in Atlassian Administration
    ///    - `atlassian/contributor` - Can access the product to view, comment, and vote only (non-billable)
    ///    - `atlassian/basic` - Can access basic product features, with no product admin permissions (non-billable)
    ///    - `atlassian/stakeholder` - Can receive incident updates and has the same product access as Customer. Non-billable but available only on Premium and Enterprise plans
    ///    - `atlassian/org-admin` - An organization admin is the highest level of admin and can complete any administrative task in Atlassian Administration
    ///    - `atlassian/site-admin` - Site admins can access Atlassian Administration and complete tasks related to the specific site they are administering.
    ///    - `atlassian/ai-access` - Can use AI features in AI-enabled apps they have access to.
    #[must_use]
    pub fn role_ids(
        mut self,
        value: impl IntoIterator<Item = impl Into<GetGroupRoleAssignmentsRequestRoleIds>>,
    ) -> Self {
        self.role_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}/role-assignments",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.directory_ids {
            config.query.push(("directoryIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_owners {
            config.query.push(("resourceOwners".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_ids {
            config.query.push(("resourceIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.role_ids {
            config.query.push(("roleIds".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MultiDirectoryGroupRoleAssignmentPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Assign a role to a group to assign all members the same role.
pub struct GrantGroupAccessRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
    resource_id: String,
    role_id: String,
}

impl<'a> GrantGroupAccessRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        resource_id: impl Into<String>,
        role_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            group_id: group_id.into(),
            resource_id: resource_id.into(),
            role_id: role_id.into(),
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}/role-assignments/assign",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        let mut body = serde_json::Map::new();

        body.insert("resourceId".to_owned(), serde_json::to_value(&self.resource_id)?);

        body.insert("roleId".to_owned(), serde_json::to_value(&self.role_id)?);

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Revoke a role from a group to remove access to an app from all members. A member can still access the app if they’re in another group that grants access to the same app.
pub struct RevokeGroupAccessRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
    resource_id: String,
    role_id: String,
}

impl<'a> RevokeGroupAccessRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        resource_id: impl Into<String>,
        role_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            group_id: group_id.into(),
            resource_id: resource_id.into(),
            role_id: role_id.into(),
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}/role-assignments/revoke",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        let mut body = serde_json::Map::new();

        body.insert("resourceId".to_owned(), serde_json::to_value(&self.resource_id)?);

        body.insert("roleId".to_owned(), serde_json::to_value(&self.role_id)?);

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Add a user to a group. This gives the user the same app access and permissions as the group. The user must be in the same directory as the group.
///
/// **Note:** Adding a user to the org-admin group through this API will return an error after the Units rollout. The org-admin group will no longer grant organization admin access after the rollout. To grant organization admin, use the [Assign organization-level role endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-users/#api-v1-orgs-orgid-users-userid-role-assignments-assign-post) instead. This applies to all organizations, not just unit organizations.
///
/// You can’t add a user to a group synced from an identity provider. Manage this group in your identity provider instead.
///
/// You can’t add a user to a group if you’ve exceeded your user limit for an app that the group grants access to. Increase your user limit or suspend another user from the app first.
pub struct AddUserToGroupRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
    account_id: String,
}

impl<'a> AddUserToGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            group_id: group_id.into(),
            account_id: account_id.into(),
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}/memberships",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        let mut body = serde_json::Map::new();

        body.insert("accountId".to_owned(), serde_json::to_value(&self.account_id)?);

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Remove a user from a group. This removes any app access and permissions granted by this group, but the user may still be in other groups that grant the same app access and permissions.
pub struct RemoveUserFromGroupRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
    account_id: String,
}

impl<'a> RemoveUserFromGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
        account_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            group_id: group_id.into(),
            account_id: account_id.into(),
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}/memberships/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id),
                crate::core::encode_path_segment(&self.account_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the details of a group.
pub struct GetGroupRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
}

impl<'a> GetGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Self {
        Self { client, org_id: org_id.into(), directory_id: directory_id.into(), group_id: group_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MultiDirectoryGroupDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete a group from a directory if you don’t need this group anymore. This removes any app access and permissions granted by this group from all members. A member can still access an app if they’re in another group that grants access to the same app.
pub struct DeleteGroupRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    group_id: String,
}

impl<'a> DeleteGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        group_id: impl Into<String>,
    ) -> Self {
        Self { client, org_id: org_id.into(), directory_id: directory_id.into(), group_id: group_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id),
                crate::core::encode_path_segment(&self.group_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the count of groups in an organization that match the supplied parameters.
pub struct GetGroupsCountRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    directory_ids: Option<Vec<String>>,
    account_ids: Option<Vec<String>>,
    group_ids: Option<Vec<String>>,
    resource_owners: Option<Vec<String>>,
    resource_ids: Option<Vec<String>>,
    search_term: Option<String>,
    role_ids: Option<Vec<GetGroupsCountRequestRoleIds>>,
}

impl<'a> GetGroupsCountRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, directory_id: impl Into<String>) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            directory_ids: None,
            account_ids: None,
            group_ids: None,
            resource_owners: None,
            resource_ids: None,
            search_term: None,
            role_ids: None,
        }
    }

    /// A list of directory IDs. The requestor must have permissions to  administer resources linked to these directories.
    #[must_use]
    pub fn directory_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.directory_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of user account IDs.
    #[must_use]
    pub fn account_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.account_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of group IDs.
    #[must_use]
    pub fn group_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.group_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of resource owners to filter the results by. Used to identify resources using their owner to which the user has at least one role assigned to.
    #[must_use]
    pub fn resource_owners(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_owners = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of resource IDs. The resource IDs should be specified  using the Atlassian Resource Identifier (ARI) format. Example ARI: `ari:cloud:jira-core::site/1`
    #[must_use]
    pub fn resource_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A search term to search the `name` field.
    #[must_use]
    pub fn search_term(mut self, value: impl Into<String>) -> Self {
        self.search_term = Some(value.into());

        self
    }

    /// A list of role IDs. The Atlassian canonical roles are used to determine the permissions of the user against resources within  the organization. The allowed roles are:
    ///    - `atlassian/user` - Can access the product, with no product admin permissions
    ///    - `atlassian/admin` - Can access the product, with product admin permissions
    ///    - `atlassian/guest` - Can only access one space you or space admins specify
    ///    - `atlassian/customer` - (Jira Service Management) Can visit help center, submit help requests, and view articles (non-billable)
    ///    - `atlassian/user-access-admin` - No product access. Can administer users and groups for this product in Atlassian Administration
    ///    - `atlassian/contributor` - Can access the product to view, comment, and vote only (non-billable)
    ///    - `atlassian/basic` - Can access basic product features, with no product admin permissions (non-billable)
    ///    - `atlassian/stakeholder` - Can receive incident updates and has the same product access as Customer. Non-billable but available only on Premium and Enterprise plans
    ///    - `atlassian/org-admin` - An organization admin is the highest level of admin and can complete any administrative task in Atlassian Administration
    ///    - `atlassian/site-admin` - Site admins can access Atlassian Administration and complete tasks related to the specific site they are administering.
    ///    - `atlassian/ai-access` - Can use AI features in AI-enabled apps they have access to.
    #[must_use]
    pub fn role_ids(mut self, value: impl IntoIterator<Item = impl Into<GetGroupsCountRequestRoleIds>>) -> Self {
        self.role_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/count",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id)
            ),
        );

        if let Some(value) = &self.directory_ids {
            config.query.push(("directoryIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.account_ids {
            config.query.push(("accountIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.group_ids {
            config.query.push(("groupIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_owners {
            config.query.push(("resourceOwners".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_ids {
            config.query.push(("resourceIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.search_term {
            config.query.push(("searchTerm".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.role_ids {
            config.query.push(("roleIds".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetGroupsCount> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns group stats for the organization.
pub struct GetGroupsStatsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
}

impl<'a> GetGroupsStatsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, directory_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), directory_id: directory_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups/stats",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MultiDirectoryGroupStats> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// **This API is deprecated and will no longer work after June 30, 2027.** Use the [Search for groups in an organization endpoint](https://developer.atlassian.com/cloud/admin/organization/rest/api-group-groups/#api-v2-orgs-orgid-directories-directoryid-groups-search-post) instead.
///
/// Returns a page of groups in an organization that match the supplied parameters.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:groups:admin`
pub struct GetGroupsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    cursor: Option<String>,
    limit: Option<i64>,
    directory_ids: Option<Vec<String>>,
    account_ids: Option<Vec<String>>,
    group_ids: Option<Vec<String>>,
    resource_owners: Option<Vec<String>>,
    resource_ids: Option<Vec<String>>,
    search_term: Option<String>,
    counts: Option<GetGroupsRequestCounts>,
    sort_by: Option<Vec<GetGroupsRequestSortBy>>,
    role_ids: Option<Vec<GetGroupsRequestRoleIds>>,
}

impl<'a> GetGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, directory_id: impl Into<String>) -> Self {
        Self {
            client,
            org_id: org_id.into(),
            directory_id: directory_id.into(),
            cursor: None,
            limit: None,
            directory_ids: None,
            account_ids: None,
            group_ids: None,
            resource_owners: None,
            resource_ids: None,
            search_term: None,
            counts: None,
            sort_by: None,
            role_ids: None,
        }
    }

    /// Sets the cursor position to retrieve the next set of results. If present, all other parameters are discarded when searching.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// The desired number of results for the search request.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// A list of directory IDs. The requestor must have permissions to  administer resources linked to these directories.
    #[must_use]
    pub fn directory_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.directory_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of user account IDs.
    #[must_use]
    pub fn account_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.account_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of group IDs.
    #[must_use]
    pub fn group_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.group_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of resource owners to filter the results by. Used to identify resources using their owner to which the user has at least one role assigned to.
    #[must_use]
    pub fn resource_owners(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_owners = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A list of resource IDs. The resource IDs should be specified  using the Atlassian Resource Identifier (ARI) format. Example ARI: `ari:cloud:jira-core::site/1`
    #[must_use]
    pub fn resource_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.resource_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A search term to search the `name` field.
    #[must_use]
    pub fn search_term(mut self, value: impl Into<String>) -> Self {
        self.search_term = Some(value.into());

        self
    }

    /// Whether to include counts of different objects associated with the group.
    #[must_use]
    pub fn counts(mut self, value: GetGroupsRequestCounts) -> Self {
        self.counts = Some(value);

        self
    }

    /// The field and direction to sort the results by. Currently, only a single field can be sorted by. If `null`, the default sorting will be used.
    #[must_use]
    pub fn sort_by(mut self, value: impl IntoIterator<Item = GetGroupsRequestSortBy>) -> Self {
        self.sort_by = Some(value.into_iter().collect());

        self
    }

    /// A list of role IDs. The Atlassian canonical roles are used to determine the permissions of the user against resources within  the organization. The allowed roles are:
    ///    - `atlassian/user` - Can access the product, with no product admin permissions
    ///    - `atlassian/admin` - Can access the product, with product admin permissions
    ///    - `atlassian/guest` - Can only access one space you or space admins specify
    ///    - `atlassian/customer` - (Jira Service Management) Can visit help center, submit help requests, and view articles (non-billable)
    ///    - `atlassian/user-access-admin` - No product access. Can administer users and groups for this product in Atlassian Administration
    ///    - `atlassian/contributor` - Can access the product to view, comment, and vote only (non-billable)
    ///    - `atlassian/basic` - Can access basic product features, with no product admin permissions (non-billable)
    ///    - `atlassian/stakeholder` - Can receive incident updates and has the same product access as Customer. Non-billable but available only on Premium and Enterprise plans
    ///    - `atlassian/org-admin` - An organization admin is the highest level of admin and can complete any administrative task in Atlassian Administration
    ///    - `atlassian/site-admin` - Site admins can access Atlassian Administration and complete tasks related to the specific site they are administering.
    ///    - `atlassian/ai-access` - Can use AI features in AI-enabled apps they have access to.
    #[must_use]
    pub fn role_ids(mut self, value: impl IntoIterator<Item = impl Into<GetGroupsRequestRoleIds>>) -> Self {
        self.role_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id)
            ),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.directory_ids {
            config.query.push(("directoryIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.account_ids {
            config.query.push(("accountIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.group_ids {
            config.query.push(("groupIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_owners {
            config.query.push(("resourceOwners".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.resource_ids {
            config.query.push(("resourceIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.search_term {
            config.query.push(("searchTerm".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.counts {
            config.query.push(("counts".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.sort_by {
            config.query.push(("sortBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.role_ids {
            config.query.push(("roleIds".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<MultiDirectoryGroupPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create a group in a directory to manage app access and permissions for multiple users together.
pub struct CreateGroupRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    directory_id: String,
    name: String,
    description: Option<String>,
}

impl<'a> CreateGroupRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        org_id: impl Into<String>,
        directory_id: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self { client, org_id: org_id.into(), directory_id: directory_id.into(), name: name.into(), description: None }
    }

    /// Describe what the group is or what it might be used for.
    #[must_use]
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/admin/v2/orgs/{}/directories/{}/groups",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.directory_id)
            ),
        );

        let mut body = serde_json::Map::new();

        body.insert("name".to_owned(), serde_json::to_value(&self.name)?);

        if let Some(value) = &self.description {
            body.insert("description".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<()> {
        self.client.send_empty(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
