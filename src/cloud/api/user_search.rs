// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// A list of project keys (case sensitive). This parameter accepts a comma-separated list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FindBulkAssignableUsersRequestProjectKeys {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The UserSearch operations.
pub struct UserSearchService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UserSearchService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of users who can be assigned issues in one or more projects. The list may be restricted to users whose attributes match a string.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that can be assigned issues in the projects. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who can be assigned issues in the projects, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for each project specified in `projectKeys`.
    pub fn find_bulk_assignable_users(
        &self,
        project_keys: FindBulkAssignableUsersRequestProjectKeys,
    ) -> FindBulkAssignableUsersRequest<'a> {
        FindBulkAssignableUsersRequest::new(self.client, project_keys)
    }

    /// Returns a list of users that can be assigned to an issue. Use this operation to find the list of users who can be assigned to:
    ///
    ///  *  a new issue, by providing the `projectKeyOrId`.
    ///  *  an updated issue, by providing the `issueKey` or `issueId`.
    ///  *  to an issue during a transition (workflow action), by providing the `issueKey` or `issueId` and the transition id in `actionDescriptorId`. You can obtain the IDs of an issue's valid transitions using the `transitions` option in the `expand` parameter of [ Get issue](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issues/#api-rest-api-3-issue-issueIdOrKey-get).
    ///
    /// In all these cases, you can pass an account ID to determine if a user can be assigned to an issue. The user is returned in the response if they can be assigned to the issue or issue transition.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that can be assigned the issue. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who can be assigned the issue, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Assign issues* [project permission](https://confluence.atlassian.com/x/yodKLg)
    pub fn find_assignable_users(&self) -> FindAssignableUsersRequest<'a> {
        FindAssignableUsersRequest::new(self.client)
    }

    /// Returns a list of users who fulfill these criteria:
    ///
    ///  *  their user attributes match a search string.
    ///  *  they have a set of permissions for a project or issue.
    ///
    /// If no search string is provided, a list of all users with the permissions is returned.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the search string and have permission for the project or issue. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the search string and have permission for the project or issue, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get users for any project.
    ///  *  *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project, to get users for that project.
    pub fn find_users_with_all_permissions(
        &self,
        permissions: impl Into<String>,
    ) -> FindUsersWithAllPermissionsRequest<'a> {
        FindUsersWithAllPermissionsRequest::new(self.client, permissions)
    }

    /// Returns a list of users whose attributes match the query term. The returned object includes the `html` field where the matched query term is highlighted with the HTML strong tag. A list of account IDs can be provided to exclude users from the results.
    ///
    /// This operation takes the users in the range defined by `maxResults`, up to the thousandth user, and then returns only the users from that range that match the query term. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the query term, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls and calls by users without the required permission return search results for an exact name match only.
    pub fn find_users_for_picker(&self, query: impl Into<String>) -> FindUsersForPickerRequest<'a> {
        FindUsersForPickerRequest::new(self.client, query)
    }

    /// Returns a list of active users that match the search string and property.
    ///
    /// This operation first applies a filter to match the search string and property, and then takes the filtered users in the range defined by `startAt` and `maxResults`, up to the thousandth user. To get all the users who match the search string and property, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls or calls by users without the required permission return empty search results.
    pub fn find_users(&self) -> FindUsersRequest<'a> {
        FindUsersRequest::new(self.client)
    }

    /// Finds users with a structured query and returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of user details.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the structured query. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the structured query, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    ///
    /// The query statements are:
    ///
    ///  *  `is assignee of PROJ` Returns the users that are assignees of at least one issue in project *PROJ*.
    ///  *  `is assignee of (PROJ-1, PROJ-2)` Returns users that are assignees on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is reporter of (PROJ-1, PROJ-2)` Returns users that are reporters on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is watcher of (PROJ-1, PROJ-2)` Returns users that are watchers on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is voter of (PROJ-1, PROJ-2)` Returns users that are voters on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is commenter of (PROJ-1, PROJ-2)` Returns users that have posted a comment on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is transitioner of (PROJ-1, PROJ-2)` Returns users that have performed a transition on issues *PROJ-1* or *PROJ-2*.
    ///  *  `[propertyKey].entity.property.path is "property value"` Returns users with the entity property value. For example, if user property `location` is set to value `{"office": {"country": "AU", "city": "Sydney"}}`, then it's possible to use `[location].office.city is "Sydney"` to match the user.
    ///
    /// The list of issues can be extended as needed, as in *(PROJ-1, PROJ-2, ... PROJ-n)*. Statements can be combined using the `AND` and `OR` operators to form more complex queries. For example:
    ///
    /// `is assignee of PROJ AND [propertyKey].entity.property.path is "property value"`
    pub fn find_users_by_query(&self, query: impl Into<String>) -> FindUsersByQueryRequest<'a> {
        FindUsersByQueryRequest::new(self.client, query)
    }

    /// Finds users with a structured query and returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of user keys.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the structured query. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the structured query, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    ///
    /// The query statements are:
    ///
    ///  *  `is assignee of PROJ` Returns the users that are assignees of at least one issue in project *PROJ*.
    ///  *  `is assignee of (PROJ-1, PROJ-2)` Returns users that are assignees on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is reporter of (PROJ-1, PROJ-2)` Returns users that are reporters on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is watcher of (PROJ-1, PROJ-2)` Returns users that are watchers on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is voter of (PROJ-1, PROJ-2)` Returns users that are voters on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is commenter of (PROJ-1, PROJ-2)` Returns users that have posted a comment on the issues *PROJ-1* or *PROJ-2*.
    ///  *  `is transitioner of (PROJ-1, PROJ-2)` Returns users that have performed a transition on issues *PROJ-1* or *PROJ-2*.
    ///  *  `[propertyKey].entity.property.path is "property value"` Returns users with the entity property value. For example, if user property `location` is set to value `{"office": {"country": "AU", "city": "Sydney"}}`, then it's possible to use `[location].office.city is "Sydney"` to match the user.
    ///
    /// The list of issues can be extended as needed, as in *(PROJ-1, PROJ-2, ... PROJ-n)*. Statements can be combined using the `AND` and `OR` operators to form more complex queries. For example:
    ///
    /// `is assignee of PROJ AND [propertyKey].entity.property.path is "property value"`
    pub fn find_user_keys_by_query(&self, query: impl Into<String>) -> FindUserKeysByQueryRequest<'a> {
        FindUserKeysByQueryRequest::new(self.client, query)
    }

    /// Returns a list of users who fulfill these criteria:
    ///
    ///  *  their user attributes match a search string.
    ///  *  they have permission to browse issues.
    ///
    /// Use this resource to find users who can browse:
    ///
    ///  *  an issue, by providing the `issueKey`.
    ///  *  any issue in a project, by providing the `projectKey`.
    ///
    /// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the search string and have permission to browse issues. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the search string and have permission to browse issues, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
    ///
    /// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls and calls by users without the required permission return empty search results.
    pub fn find_users_with_browse_permission(&self) -> FindUsersWithBrowsePermissionRequest<'a> {
        FindUsersWithBrowsePermissionRequest::new(self.client)
    }
}

/// Returns a list of users who can be assigned issues in one or more projects. The list may be restricted to users whose attributes match a string.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that can be assigned issues in the projects. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who can be assigned issues in the projects, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for each project specified in `projectKeys`.
#[derive(Clone)]
pub struct FindBulkAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    account_id: Option<String>,
    project_keys: FindBulkAssignableUsersRequestProjectKeys,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> FindBulkAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client, project_keys: FindBulkAssignableUsersRequestProjectKeys) -> Self {
        Self { client, project_keys, query: None, account_id: None, start_at: None, max_results: None }
    }

    /// A query string that is matched against user attributes, such as `displayName` and `emailAddress`, to find relevant users. The string can match the prefix of the attribute's value. For example, *query=john* matches a user with a `displayName` of *John Smith* and a user with an `emailAddress` of *johnson@example.com*. Required, unless `accountId` is specified.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// A query string that is matched exactly against user `accountId`. Required, unless `query` is specified.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/user/assignable/multiProjectSearch".to_owned(),
        );

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.query.push(("projectKeys".to_owned(), crate::core::QueryValue::from_serializable(&self.project_keys)?));

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users that can be assigned to an issue. Use this operation to find the list of users who can be assigned to:
///
///  *  a new issue, by providing the `projectKeyOrId`.
///  *  an updated issue, by providing the `issueKey` or `issueId`.
///  *  to an issue during a transition (workflow action), by providing the `issueKey` or `issueId` and the transition id in `actionDescriptorId`. You can obtain the IDs of an issue's valid transitions using the `transitions` option in the `expand` parameter of [ Get issue](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issues/#api-rest-api-3-issue-issueIdOrKey-get).
///
/// In all these cases, you can pass an account ID to determine if a user can be assigned to an issue. The user is returned in the response if they can be assigned to the issue or issue transition.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that can be assigned the issue. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who can be assigned the issue, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Assign issues* [project permission](https://confluence.atlassian.com/x/yodKLg)
#[derive(Clone)]
pub struct FindAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    session_id: Option<String>,
    account_id: Option<String>,
    project: Option<String>,
    issue_key: Option<String>,
    issue_id: Option<String>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    action_descriptor_id: Option<i64>,
    recommend: Option<bool>,
    account_type: Option<Vec<String>>,
    app_type: Option<Vec<String>>,
}

impl<'a> FindAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            query: None,
            session_id: None,
            account_id: None,
            project: None,
            issue_key: None,
            issue_id: None,
            start_at: None,
            max_results: None,
            action_descriptor_id: None,
            recommend: None,
            account_type: None,
            app_type: None,
        }
    }

    /// A query string that is matched against user attributes, such as `displayName`, and `emailAddress`, to find relevant users. The string can match the prefix of the attribute's value. For example, *query=john* matches a user with a `displayName` of *John Smith* and a user with an `emailAddress` of *johnson@example.com*. Required, unless `username` or `accountId` is specified.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// The sessionId of this request. SessionId is the same until the assignee is set.
    #[must_use]
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());

        self
    }

    /// A query string that is matched exactly against user `accountId`. Required, unless `query` is specified.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The project ID or project key (case sensitive). Required, unless `issueKey` or `issueId` is specified.
    #[must_use]
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());

        self
    }

    /// The key of the issue. Required, unless `issueId` or `project` is specified.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The ID of the issue. Required, unless `issueKey` or `project` is specified.
    #[must_use]
    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return. This operation may return less than the maximum number of items even if more are available. The operation fetches users up to the maximum and then, from the fetched users, returns only the users that can be assigned to the issue.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The ID of the transition.
    #[must_use]
    pub fn action_descriptor_id(mut self, value: i64) -> Self {
        self.action_descriptor_id = Some(value);

        self
    }

    #[must_use]
    pub fn recommend(mut self, value: bool) -> Self {
        self.recommend = Some(value);

        self
    }

    #[must_use]
    pub fn account_type(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.account_type = Some(value.into_iter().map(Into::into).collect());

        self
    }

    #[must_use]
    pub fn app_type(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.app_type = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/assignable/search".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.session_id {
            config.query.push(("sessionId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project {
            config.query.push(("project".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_id {
            config.query.push(("issueId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.action_descriptor_id {
            config.query.push(("actionDescriptorId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.recommend {
            config.query.push(("recommend".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.account_type {
            config.query.push(("accountType".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.app_type {
            config.query.push(("appType".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users who fulfill these criteria:
///
///  *  their user attributes match a search string.
///  *  they have a set of permissions for a project or issue.
///
/// If no search string is provided, a list of all users with the permissions is returned.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the search string and have permission for the project or issue. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the search string and have permission for the project or issue, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get users for any project.
///  *  *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project, to get users for that project.
#[derive(Clone)]
pub struct FindUsersWithAllPermissionsRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    account_id: Option<String>,
    permissions: String,
    issue_key: Option<String>,
    project_key: Option<String>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> FindUsersWithAllPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client, permissions: impl Into<String>) -> Self {
        Self {
            client,
            permissions: permissions.into(),
            query: None,
            account_id: None,
            issue_key: None,
            project_key: None,
            start_at: None,
            max_results: None,
        }
    }

    /// A query string that is matched against user attributes, such as `displayName` and `emailAddress`, to find relevant users. The string can match the prefix of the attribute's value. For example, *query=john* matches a user with a `displayName` of *John Smith* and a user with an `emailAddress` of *johnson@example.com*. Required, unless `accountId` is specified.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// A query string that is matched exactly against user `accountId`. Required, unless `query` is specified.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The issue key for the issue.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The project key for the project (case sensitive).
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/permission/search".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        config.query.push(("permissions".to_owned(), crate::core::QueryValue::Scalar(self.permissions.clone())));

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users whose attributes match the query term. The returned object includes the `html` field where the matched query term is highlighted with the HTML strong tag. A list of account IDs can be provided to exclude users from the results.
///
/// This operation takes the users in the range defined by `maxResults`, up to the thousandth user, and then returns only the users from that range that match the query term. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the query term, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls and calls by users without the required permission return search results for an exact name match only.
#[derive(Clone)]
pub struct FindUsersForPickerRequest<'a> {
    client: &'a crate::core::Client,
    query: String,
    max_results: Option<i64>,
    show_avatar: Option<bool>,
    exclude_account_ids: Option<Vec<String>>,
    avatar_size: Option<String>,
    exclude_connect_users: Option<bool>,
}

impl<'a> FindUsersForPickerRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>) -> Self {
        Self {
            client,
            query: query.into(),
            max_results: None,
            show_avatar: None,
            exclude_account_ids: None,
            avatar_size: None,
            exclude_connect_users: None,
        }
    }

    /// The maximum number of items to return. The total number of matched users is returned in `total`.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Include the URI to the user's avatar.
    #[must_use]
    pub fn show_avatar(mut self, value: bool) -> Self {
        self.show_avatar = Some(value);

        self
    }

    /// A list of account IDs to exclude from the search results. This parameter accepts a comma-separated list. Multiple account IDs can also be provided using an ampersand-separated list. For example, `excludeAccountIds=5b10a2844c20165700ede21g,5b10a0effa615349cb016cd8&excludeAccountIds=5b10ac8d82e05b22cc7d4ef5`. Cannot be provided with `exclude`.
    #[must_use]
    pub fn exclude_account_ids(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude_account_ids = Some(value.into_iter().map(Into::into).collect());

        self
    }

    #[must_use]
    pub fn avatar_size(mut self, value: impl Into<String>) -> Self {
        self.avatar_size = Some(value.into());

        self
    }

    #[must_use]
    pub fn exclude_connect_users(mut self, value: bool) -> Self {
        self.exclude_connect_users = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/picker".to_owned());

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.show_avatar {
            config.query.push(("showAvatar".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.exclude_account_ids {
            config.query.push(("excludeAccountIds".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.avatar_size {
            config.query.push(("avatarSize".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.exclude_connect_users {
            config.query.push(("excludeConnectUsers".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<FoundUsers> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of active users that match the search string and property.
///
/// This operation first applies a filter to match the search string and property, and then takes the filtered users in the range defined by `startAt` and `maxResults`, up to the thousandth user. To get all the users who match the search string and property, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// This operation can be accessed anonymously.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls or calls by users without the required permission return empty search results.
#[derive(Clone)]
pub struct FindUsersRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    username: Option<String>,
    account_id: Option<String>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    property: Option<String>,
}

impl<'a> FindUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            query: None,
            username: None,
            account_id: None,
            start_at: None,
            max_results: None,
            property: None,
        }
    }

    /// A query string that is matched against user attributes ( `displayName`, and `emailAddress`) to find relevant users. The string can match the prefix of the attribute's value. For example, *query=john* matches a user with a `displayName` of *John Smith* and a user with an `emailAddress` of *johnson@example.com*. Required, unless `accountId` or `property` is specified.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// A query string that is matched exactly against a user `accountId`. Required, unless `query` or `property` is specified.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The index of the first item to return in a page of filtered results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// A query string used to search properties. Property keys are specified by path, so property keys containing dot (.) or equals (=) characters cannot be used. The query string cannot be specified using a JSON object. Example: To search for the value of `nested` from `{"something":{"nested":1,"other":2}}` use `thepropertykey.something.nested=1`. Required, unless `accountId` or `query` is specified.
    #[must_use]
    pub fn property(mut self, value: impl Into<String>) -> Self {
        self.property = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/search".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.property {
            config.query.push(("property".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Finds users with a structured query and returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of user details.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the structured query. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the structured query, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
///
/// The query statements are:
///
///  *  `is assignee of PROJ` Returns the users that are assignees of at least one issue in project *PROJ*.
///  *  `is assignee of (PROJ-1, PROJ-2)` Returns users that are assignees on the issues *PROJ-1* or *PROJ-2*.
///  *  `is reporter of (PROJ-1, PROJ-2)` Returns users that are reporters on the issues *PROJ-1* or *PROJ-2*.
///  *  `is watcher of (PROJ-1, PROJ-2)` Returns users that are watchers on the issues *PROJ-1* or *PROJ-2*.
///  *  `is voter of (PROJ-1, PROJ-2)` Returns users that are voters on the issues *PROJ-1* or *PROJ-2*.
///  *  `is commenter of (PROJ-1, PROJ-2)` Returns users that have posted a comment on the issues *PROJ-1* or *PROJ-2*.
///  *  `is transitioner of (PROJ-1, PROJ-2)` Returns users that have performed a transition on issues *PROJ-1* or *PROJ-2*.
///  *  `[propertyKey].entity.property.path is "property value"` Returns users with the entity property value. For example, if user property `location` is set to value `{"office": {"country": "AU", "city": "Sydney"}}`, then it's possible to use `[location].office.city is "Sydney"` to match the user.
///
/// The list of issues can be extended as needed, as in *(PROJ-1, PROJ-2, ... PROJ-n)*. Statements can be combined using the `AND` and `OR` operators to form more complex queries. For example:
///
/// `is assignee of PROJ AND [propertyKey].entity.property.path is "property value"`
#[derive(Clone)]
pub struct FindUsersByQueryRequest<'a> {
    client: &'a crate::core::Client,
    query: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> FindUsersByQueryRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>) -> Self {
        Self { client, query: query.into(), start_at: None, max_results: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/search/query".to_owned());

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<DashboardUser>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start_at = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Finds users with a structured query and returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of user keys.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the structured query. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the structured query, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg).
///
/// The query statements are:
///
///  *  `is assignee of PROJ` Returns the users that are assignees of at least one issue in project *PROJ*.
///  *  `is assignee of (PROJ-1, PROJ-2)` Returns users that are assignees on the issues *PROJ-1* or *PROJ-2*.
///  *  `is reporter of (PROJ-1, PROJ-2)` Returns users that are reporters on the issues *PROJ-1* or *PROJ-2*.
///  *  `is watcher of (PROJ-1, PROJ-2)` Returns users that are watchers on the issues *PROJ-1* or *PROJ-2*.
///  *  `is voter of (PROJ-1, PROJ-2)` Returns users that are voters on the issues *PROJ-1* or *PROJ-2*.
///  *  `is commenter of (PROJ-1, PROJ-2)` Returns users that have posted a comment on the issues *PROJ-1* or *PROJ-2*.
///  *  `is transitioner of (PROJ-1, PROJ-2)` Returns users that have performed a transition on issues *PROJ-1* or *PROJ-2*.
///  *  `[propertyKey].entity.property.path is "property value"` Returns users with the entity property value. For example, if user property `location` is set to value `{"office": {"country": "AU", "city": "Sydney"}}`, then it's possible to use `[location].office.city is "Sydney"` to match the user.
///
/// The list of issues can be extended as needed, as in *(PROJ-1, PROJ-2, ... PROJ-n)*. Statements can be combined using the `AND` and `OR` operators to form more complex queries. For example:
///
/// `is assignee of PROJ AND [propertyKey].entity.property.path is "property value"`
#[derive(Clone)]
pub struct FindUserKeysByQueryRequest<'a> {
    client: &'a crate::core::Client,
    query: String,
    start_at: Option<i64>,
    max_result: Option<i64>,
}

impl<'a> FindUserKeysByQueryRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>) -> Self {
        Self { client, query: query.into(), start_at: None, max_result: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_result(mut self, value: i64) -> Self {
        self.max_result = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/search/query/key".to_owned());

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_result {
            config.query.push(("maxResult".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<UserKey>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start_at = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<UserKey>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users who fulfill these criteria:
///
///  *  their user attributes match a search string.
///  *  they have permission to browse issues.
///
/// Use this resource to find users who can browse:
///
///  *  an issue, by providing the `issueKey`.
///  *  any issue in a project, by providing the `projectKey`.
///
/// This operation takes the users in the range defined by `startAt` and `maxResults`, up to the thousandth user, and then returns only the users from that range that match the search string and have permission to browse issues. This means the operation usually returns fewer users than specified in `maxResults`. To get all the users who match the search string and have permission to browse issues, use [Get all users](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-users/#api-rest-api-3-users-search-get) and filter the records in your code.
///
/// Privacy controls are applied to the response based on the users' preferences. This could mean, for example, that the user's email address is hidden. See the [Profile visibility overview](https://developer.atlassian.com/cloud/jira/platform/profile-visibility/) for more details.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse users and groups* [global permission](https://confluence.atlassian.com/x/x4dKLg). Anonymous calls and calls by users without the required permission return empty search results.
#[derive(Clone)]
pub struct FindUsersWithBrowsePermissionRequest<'a> {
    client: &'a crate::core::Client,
    query: Option<String>,
    account_id: Option<String>,
    issue_key: Option<String>,
    project_key: Option<String>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> FindUsersWithBrowsePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            query: None,
            account_id: None,
            issue_key: None,
            project_key: None,
            start_at: None,
            max_results: None,
        }
    }

    /// A query string that is matched against user attributes, such as `displayName` and `emailAddress`, to find relevant users. The string can match the prefix of the attribute's value. For example, *query=john* matches a user with a `displayName` of *John Smith* and a user with an `emailAddress` of *johnson@example.com*. Required, unless `accountId` is specified.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// A query string that is matched exactly against user `accountId`. Required, unless `query` is specified.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The issue key for the issue. Required, unless `projectKey` is specified.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The project key for the project (case sensitive). Required, unless `issueKey` is specified.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/viewissue/search".to_owned());

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<DashboardUser>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
