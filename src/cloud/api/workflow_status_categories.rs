// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The WorkflowStatusCategories operations.
pub struct WorkflowStatusCategoriesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowStatusCategoriesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all status categories.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_status_categories(&self) -> GetStatusCategoriesRequest<'a> {
        GetStatusCategoriesRequest::new(self.client)
    }

    /// Returns a status category. Status categories provided a mechanism for categorizing [statuses](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-status/#api-rest-api-3-status-idOrName-get).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_status_category(&self, id_or_key: impl Into<String>) -> GetStatusCategoryRequest<'a> {
        GetStatusCategoryRequest::new(self.client, id_or_key)
    }
}

/// Returns a list of all status categories.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetStatusCategoriesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetStatusCategoriesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/statuscategory".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<StatusCategory>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a status category. Status categories provided a mechanism for categorizing [statuses](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-status/#api-rest-api-3-status-idOrName-get).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetStatusCategoryRequest<'a> {
    client: &'a crate::core::Client,
    id_or_key: String,
}

impl<'a> GetStatusCategoryRequest<'a> {
    fn new(client: &'a crate::core::Client, id_or_key: impl Into<String>) -> Self {
        Self { client, id_or_key: id_or_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/statuscategory/{}", crate::core::encode_path_segment(&self.id_or_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<StatusCategory> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
