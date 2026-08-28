// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectEmail operations.
pub struct ProjectEmailService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectEmailService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    pub fn get_project_email(&self, project_id: i64) -> GetProjectEmailRequest<'a> {
        GetProjectEmailRequest::new(self.client, project_id)
    }

    /// Sets the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).
    ///
    /// If `emailAddress` is an empty string, the default email address is restored.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
    pub fn update_project_email(
        &self,
        project_id: i64,
        project_email_address: ProjectEmailAddress,
    ) -> UpdateProjectEmailRequest<'a> {
        UpdateProjectEmailRequest::new(self.client, project_id, project_email_address)
    }
}

/// Returns the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
pub struct GetProjectEmailRequest<'a> {
    client: &'a crate::core::Client,
    project_id: i64,
}

impl<'a> GetProjectEmailRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id: i64) -> Self {
        Self { client, project_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/project/{}/email", self.project_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ProjectEmailAddress> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the [project's sender email address](https://confluence.atlassian.com/x/dolKLg).
///
/// If `emailAddress` is an empty string, the default email address is restored.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) or *Administer Projects* [project permission.](https://confluence.atlassian.com/x/yodKLg)
pub struct UpdateProjectEmailRequest<'a> {
    client: &'a crate::core::Client,
    project_id: i64,
    project_email_address: ProjectEmailAddress,
}

impl<'a> UpdateProjectEmailRequest<'a> {
    fn new(client: &'a crate::core::Client, project_id: i64, project_email_address: ProjectEmailAddress) -> Self {
        Self { client, project_id, project_email_address }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/project/{}/email", self.project_id),
        );

        let body = match serde_json::to_value(&self.project_email_address)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

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
