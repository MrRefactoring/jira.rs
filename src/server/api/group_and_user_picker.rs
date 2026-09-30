// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The GroupAndUserPicker operations.
pub struct GroupAndUserPickerService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GroupAndUserPickerService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of users and groups matching query with highlighting
    pub fn find_users_and_groups(&self) -> FindUsersAndGroupsRequest<'a> {
        FindUsersAndGroupsRequest::new(self.client)
    }
}

/// Returns a list of users and groups matching query with highlighting
#[derive(Clone)]
pub struct FindUsersAndGroupsRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: Option<String>,
    max_results: Option<String>,
    query: Option<String>,
    show_avatar: Option<String>,
    project_id: Option<String>,
    field_id: Option<String>,
}

impl<'a> FindUsersAndGroupsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            issue_type_id: None,
            max_results: None,
            query: None,
            show_avatar: None,
            project_id: None,
            field_id: None,
        }
    }

    /// The list of issue type ids to further restrict the search
    #[must_use]
    pub fn issue_type_id(mut self, value: impl Into<String>) -> Self {
        self.issue_type_id = Some(value.into());

        self
    }

    /// The maximum number of users to return
    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    /// A string used to search username, Name or e-mail address
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// Show avatar
    #[must_use]
    pub fn show_avatar(mut self, value: impl Into<String>) -> Self {
        self.show_avatar = Some(value.into());

        self
    }

    /// The list of project ids to further restrict the search
    #[must_use]
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());

        self
    }

    /// The custom field id
    #[must_use]
    pub fn field_id(mut self, value: impl Into<String>) -> Self {
        self.field_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/groupuserpicker".to_owned());

        if let Some(value) = &self.issue_type_id {
            config.query.push(("issueTypeId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.show_avatar {
            config.query.push(("showAvatar".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.field_id {
            config.query.push(("fieldId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UsersAndGroups> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
