// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Groups operations.
pub struct GroupsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GroupsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a group.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
    pub fn create_group(&self, add_group: AddGroup) -> CreateGroupRequest<'a> {
        CreateGroupRequest::new(self.client, add_group)
    }

    /// Deletes a group.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* strategic [group](https://confluence.atlassian.com/x/24xjL)).
    pub fn remove_group(&self) -> RemoveGroupRequest<'a> {
        RemoveGroupRequest::new(self.client)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all users in a group.
    ///
    /// Note that users are ordered by username, however the username is not returned in the results due to privacy reasons.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
    ///
    ///  *  *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_users_from_group(&self) -> GetUsersFromGroupRequest<'a> {
        GetUsersFromGroupRequest::new(self.client)
    }

    /// Adds a user to a group.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
    pub fn add_user_to_group(&self, update_user_to_group: UpdateUserToGroup) -> AddUserToGroupRequest<'a> {
        AddUserToGroupRequest::new(self.client, update_user_to_group)
    }

    /// Removes a user from a group.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
    pub fn remove_user_from_group(&self, account_id: impl Into<String>) -> RemoveUserFromGroupRequest<'a> {
        RemoveUserFromGroupRequest::new(self.client, account_id)
    }

    /// Returns a list of groups whose names contain a query string. A list of group names can be provided to exclude groups from the results.
    ///
    /// The primary use case for this resource is to populate a group picker suggestions list. To this end, the returned object includes the `html` field where the matched query term is highlighted in the group name with the HTML strong tag. Also, the groups list is wrapped in a response object that contains a header for use in the picker, specifically *Showing X of Y matching groups*.
    ///
    /// The list returns with the groups sorted. If no groups match the list criteria, an empty list is returned.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg). Anonymous calls and calls by users without the required permission return an empty list.
    ///
    /// *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Without this permission, calls where query is not an exact match to an existing group will return an empty list.
    pub fn find_groups(&self) -> FindGroupsRequest<'a> {
        FindGroupsRequest::new(self.client)
    }
}

/// Creates a group.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
#[derive(Clone)]
pub struct CreateGroupRequest<'a> {
    client: &'a crate::core::Client,
    add_group: AddGroup,
}

impl<'a> CreateGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, add_group: AddGroup) -> Self {
        Self { client, add_group }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/group".to_owned());

        let body = match serde_json::to_value(&self.add_group)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Group> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a group.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* strategic [group](https://confluence.atlassian.com/x/24xjL)).
#[derive(Clone)]
pub struct RemoveGroupRequest<'a> {
    client: &'a crate::core::Client,
    groupname: Option<String>,
    group_id: Option<String>,
    swap_group: Option<String>,
    swap_group_id: Option<String>,
}

impl<'a> RemoveGroupRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, groupname: None, group_id: None, swap_group: None, swap_group_id: None }
    }

    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// The ID of the group. This parameter cannot be used with the `groupname` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// As a group's name can change, use of `swapGroupId` is recommended to identify a group.
    /// The group to transfer restrictions to. Only comments and worklogs are transferred. If restrictions are not transferred, comments and worklogs are inaccessible after the deletion. This parameter cannot be used with the `swapGroupId` parameter.
    #[must_use]
    pub fn swap_group(mut self, value: impl Into<String>) -> Self {
        self.swap_group = Some(value.into());

        self
    }

    /// The ID of the group to transfer restrictions to. Only comments and worklogs are transferred. If restrictions are not transferred, comments and worklogs are inaccessible after the deletion. This parameter cannot be used with the `swapGroup` parameter.
    #[must_use]
    pub fn swap_group_id(mut self, value: impl Into<String>) -> Self {
        self.swap_group_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/group".to_owned());

        if let Some(value) = &self.groupname {
            config.query.push(("groupname".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.swap_group {
            config.query.push(("swapGroup".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.swap_group_id {
            config.query.push(("swapGroupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all users in a group.
///
/// Note that users are ordered by username, however the username is not returned in the results due to privacy reasons.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
///
///  *  *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetUsersFromGroupRequest<'a> {
    client: &'a crate::core::Client,
    groupname: Option<String>,
    group_id: Option<String>,
    include_inactive_users: Option<bool>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetUsersFromGroupRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            groupname: None,
            group_id: None,
            include_inactive_users: None,
            start_at: None,
            max_results: None,
        }
    }

    /// As a group's name can change, use of `groupId` is recommended to identify a group.
    /// The name of the group. This parameter cannot be used with the `groupId` parameter.
    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// Include inactive users.
    #[must_use]
    pub fn include_inactive_users(mut self, value: bool) -> Self {
        self.include_inactive_users = Some(value);

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page (number should be between 1 and 50).
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/group/member".to_owned());

        if let Some(value) = &self.groupname {
            config.query.push(("groupname".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.include_inactive_users {
            config.query.push(("includeInactiveUsers".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<UserDetails>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start_at = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<UserDetails>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Adds a user to a group.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
#[derive(Clone)]
pub struct AddUserToGroupRequest<'a> {
    client: &'a crate::core::Client,
    groupname: Option<String>,
    group_id: Option<String>,
    update_user_to_group: UpdateUserToGroup,
}

impl<'a> AddUserToGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, update_user_to_group: UpdateUserToGroup) -> Self {
        Self { client, update_user_to_group, groupname: None, group_id: None }
    }

    /// As a group's name can change, use of `groupId` is recommended to identify a group.
    /// The name of the group. This parameter cannot be used with the `groupId` parameter.
    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/group/user".to_owned());

        if let Some(value) = &self.groupname {
            config.query.push(("groupname".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        let body = match serde_json::to_value(&self.update_user_to_group)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Group> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes a user from a group.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Site administration (that is, member of the *site-admin* [group](https://confluence.atlassian.com/x/24xjL)).
#[derive(Clone)]
pub struct RemoveUserFromGroupRequest<'a> {
    client: &'a crate::core::Client,
    groupname: Option<String>,
    group_id: Option<String>,
    account_id: String,
}

impl<'a> RemoveUserFromGroupRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: impl Into<String>) -> Self {
        Self { client, account_id: account_id.into(), groupname: None, group_id: None }
    }

    /// As a group's name can change, use of `groupId` is recommended to identify a group.
    /// The name of the group. This parameter cannot be used with the `groupId` parameter.
    #[must_use]
    pub fn groupname(mut self, value: impl Into<String>) -> Self {
        self.groupname = Some(value.into());

        self
    }

    /// The ID of the group. This parameter cannot be used with the `groupName` parameter.
    #[must_use]
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/group/user".to_owned());

        if let Some(value) = &self.groupname {
            config.query.push(("groupname".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.group_id {
            config.query.push(("groupId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(self.account_id.clone())));

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

/// Returns a list of groups whose names contain a query string. A list of group names can be provided to exclude groups from the results.
///
/// The primary use case for this resource is to populate a group picker suggestions list. To this end, the returned object includes the `html` field where the matched query term is highlighted in the group name with the HTML strong tag. Also, the groups list is wrapped in a response object that contains a header for use in the picker, specifically *Showing X of Y matching groups*.
///
/// The list returns with the groups sorted. If no groups match the list criteria, an empty list is returned.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg). Anonymous calls and calls by users without the required permission return an empty list.
///
/// *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Without this permission, calls where query is not an exact match to an existing group will return an empty list.
#[derive(Clone)]
pub struct FindGroupsRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    exclude: Option<Vec<String>>,
    exclude_id: Option<Vec<String>>,
    max_results: Option<i64>,
    case_insensitive: Option<bool>,
}

impl<'a> FindGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, query: None, exclude: None, exclude_id: None, max_results: None, case_insensitive: None }
    }

    /// The string to find in group names.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// As a group's name can change, use of `excludeGroupIds` is recommended to identify a group.
    /// A group to exclude from the result. To exclude multiple groups, provide an ampersand-separated list. For example, `exclude=group1&exclude=group2`. This parameter cannot be used with the `excludeGroupIds` parameter.
    #[must_use]
    pub fn exclude(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// A group ID to exclude from the result. To exclude multiple groups, provide an ampersand-separated list. For example, `excludeId=group1-id&excludeId=group2-id`. This parameter cannot be used with the `excludeGroups` parameter.
    #[must_use]
    pub fn exclude_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude_id = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The maximum number of groups to return. The maximum number of groups that can be returned is limited by the system property `jira.ajax.autocomplete.limit`.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Whether the search for groups should be case insensitive.
    #[must_use]
    pub fn case_insensitive(mut self, value: bool) -> Self {
        self.case_insensitive = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/groups/picker".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.exclude {
            config.query.push(("exclude".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.exclude_id {
            config.query.push(("excludeId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.case_insensitive {
            config.query.push(("caseInsensitive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FoundGroups> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
