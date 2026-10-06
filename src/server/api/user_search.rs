// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The UserSearch operations.
pub struct UserSearchService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UserSearchService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of users that match the search string and can be assigned issues for all the given projects.
    pub fn find_bulk_assignable_users(
        &self,
        project_keys: impl IntoIterator<Item = impl Into<String>>,
    ) -> FindBulkAssignableUsersRequest<'a> {
        FindBulkAssignableUsersRequest::new(self.client, project_keys)
    }

    /// Returns a list of users that match the search string. This resource cannot be accessed anonymously. Please note that this resource should be called with an issue key when a list of assignable users is retrieved. For create only a project key should be supplied. The list of assignable users may be incorrect if it's called with the project key for editing.
    pub fn find_assignable_users(&self) -> FindAssignableUsersRequest<'a> {
        FindAssignableUsersRequest::new(self.client)
    }

    /// Returns a list of users matching query with highlighting.
    pub fn find_users_for_picker(&self, query: impl Into<String>) -> FindUsersForPickerRequest<'a> {
        FindUsersForPickerRequest::new(self.client, query)
    }

    /// Finds users.
    pub fn find_users(&self) -> FindUsersRequest<'a> {
        FindUsersRequest::new(self.client)
    }

    /// Returns a list of active users that match the search string. This resource cannot be accessed anonymously and requires the Browse Users global permission. Given an issue key this resource will provide a list of users that match the search string and have the browse issue permission for the issue provided.
    pub fn find_users_with_browse_permission(&self) -> FindUsersWithBrowsePermissionRequest<'a> {
        FindUsersWithBrowsePermissionRequest::new(self.client)
    }
}

/// Returns a list of users that match the search string and can be assigned issues for all the given projects.
#[derive(Clone)]
pub struct FindBulkAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    project_keys: Vec<String>,
    username: Option<String>,
}

impl<'a> FindBulkAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client, project_keys: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            client,
            project_keys: project_keys.into_iter().map(Into::into).collect(),
            max_results: None,
            username: None,
        }
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/user/assignable/multiProjectSearch".to_owned(),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.query.push(("projectKeys".to_owned(), crate::core::QueryValue::Scalar(self.project_keys.join(","))));

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users that match the search string. This resource cannot be accessed anonymously. Please note that this resource should be called with an issue key when a list of assignable users is retrieved. For create only a project key should be supplied. The list of assignable users may be incorrect if it's called with the project key for editing.
#[derive(Clone)]
pub struct FindAssignableUsersRequest<'a> {
    client: &'a crate::core::Client,
    issue_key: Option<String>,
    max_results: Option<i64>,
    project: Option<String>,
    action_descriptor_id: Option<i64>,
    username: Option<String>,
}

impl<'a> FindAssignableUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, issue_key: None, max_results: None, project: None, action_descriptor_id: None, username: None }
    }

    /// the issue key for the issue being edited we need to find assignable users for.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    #[must_use]
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());

        self
    }

    #[must_use]
    pub fn action_descriptor_id(mut self, value: i64) -> Self {
        self.action_descriptor_id = Some(value);

        self
    }

    /// the username
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/assignable/search".to_owned());

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.project {
            config.query.push(("project".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.action_descriptor_id {
            config.query.push(("actionDescriptorId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of users matching query with highlighting.
#[derive(Clone)]
pub struct FindUsersForPickerRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<i64>,
    query: String,
    exclude: Option<Vec<String>>,
    show_avatar: Option<bool>,
}

impl<'a> FindUsersForPickerRequest<'a> {
    fn new(client: &'a crate::core::Client, query: impl Into<String>) -> Self {
        Self { client, query: query.into(), max_results: None, exclude: None, show_avatar: None }
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// List of users to be excluded from the search results
    #[must_use]
    pub fn exclude(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.exclude = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// If true, then avatars are included in the results
    #[must_use]
    pub fn show_avatar(mut self, value: bool) -> Self {
        self.show_avatar = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/picker".to_owned());

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(self.query.clone())));

        if let Some(value) = &self.exclude {
            config.query.push(("exclude".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.show_avatar {
            config.query.push(("showAvatar".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
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

/// Finds users.
#[derive(Clone)]
pub struct FindUsersRequest<'a> {
    client: &'a crate::core::Client,
    include_inactive: Option<bool>,
    max_results: Option<i64>,
    include_active: Option<bool>,
    start_at: Option<i64>,
    username: Option<String>,
}

impl<'a> FindUsersRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, include_inactive: None, max_results: None, include_active: None, start_at: None, username: None }
    }

    /// If true, then inactive users are included in the results (default false)
    #[must_use]
    pub fn include_inactive(mut self, value: bool) -> Self {
        self.include_inactive = Some(value);

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// If true, then active users are included in the results (default true)
    #[must_use]
    pub fn include_active(mut self, value: bool) -> Self {
        self.include_active = Some(value);

        self
    }

    /// The index of the first user to return (0-based). Please note that the startAt parameter will be deprecated in a future release of Jira 10.3.x
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// A query string used to search username, name or e-mail address
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/search".to_owned());

        if let Some(value) = &self.include_inactive {
            config.query.push(("includeInactive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.include_active {
            config.query.push(("includeActive".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<User>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.clone().send().await?;
            let items = page;
            Ok(crate::core::PageStep::offset(items, position, false, None, request.max_results))
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of active users that match the search string. This resource cannot be accessed anonymously and requires the Browse Users global permission. Given an issue key this resource will provide a list of users that match the search string and have the browse issue permission for the issue provided.
#[derive(Clone)]
pub struct FindUsersWithBrowsePermissionRequest<'a> {
    client: &'a crate::core::Client,
    project_key: Option<String>,
    issue_key: Option<String>,
    max_results: Option<i64>,
    username: Option<String>,
}

impl<'a> FindUsersWithBrowsePermissionRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, project_key: None, issue_key: None, max_results: None, username: None }
    }

    /// the optional project key to search for users with if no issueKey is supplied.
    #[must_use]
    pub fn project_key(mut self, value: impl Into<String>) -> Self {
        self.project_key = Some(value.into());

        self
    }

    /// the issue key for the issue being edited we need to find viewable users for.
    #[must_use]
    pub fn issue_key(mut self, value: impl Into<String>) -> Self {
        self.issue_key = Some(value.into());

        self
    }

    /// The maximum number of users to return (defaults to 50). The maximum allowed value is 100 (The combination of maxResults and startAt is limited to the first 100 results). If you specify a value that is higher than this number, your search results will be truncated. If you send a request with startAt=98 and maxResults=20, it will only return 2 users.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// the username filter, no users returned if left blank
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/viewissue/search".to_owned());

        if let Some(value) = &self.project_key {
            config.query.push(("projectKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_key {
            config.query.push(("issueKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<User>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
