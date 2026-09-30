// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The ProjectKeyAndNameValidation operations.
pub struct ProjectKeyAndNameValidationService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProjectKeyAndNameValidationService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Validates a project key by confirming the key is a valid string and not in use.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn validate_project_key(&self) -> ValidateProjectKeyRequest<'a> {
        ValidateProjectKeyRequest::new(self.client)
    }

    /// Validates a project key and, if the key is invalid or in use, generates a valid random string for the project key.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_valid_project_key(&self) -> GetValidProjectKeyRequest<'a> {
        GetValidProjectKeyRequest::new(self.client)
    }

    /// Checks that a project name isn't in use. If the name isn't in use, the passed string is returned. If the name is in use, this operation attempts to generate a valid project name based on the one supplied, usually by adding a sequence number. If a valid project name cannot be generated, a 404 response is returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
    pub fn get_valid_project_name(&self, name: impl Into<String>) -> GetValidProjectNameRequest<'a> {
        GetValidProjectNameRequest::new(self.client, name)
    }
}

/// Validates a project key by confirming the key is a valid string and not in use.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct ValidateProjectKeyRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
}

impl<'a> ValidateProjectKeyRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None }
    }

    /// The project key.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/projectvalidate/key".to_owned());

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ErrorCollection> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Validates a project key and, if the key is invalid or in use, generates a valid random string for the project key.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetValidProjectKeyRequest<'a> {
    client: &'a crate::core::Client,
    key: Option<String>,
}

impl<'a> GetValidProjectKeyRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, key: None }
    }

    /// The project key.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/projectvalidate/validProjectKey".to_owned(),
        );

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Checks that a project name isn't in use. If the name isn't in use, the passed string is returned. If the name is in use, this operation attempts to generate a valid project name based on the one supplied, usually by adding a sequence number. If a valid project name cannot be generated, a 404 response is returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** None.
#[derive(Clone)]
pub struct GetValidProjectNameRequest<'a> {
    client: &'a crate::core::Client,
    name: String,
}

impl<'a> GetValidProjectNameRequest<'a> {
    fn new(client: &'a crate::core::Client, name: impl Into<String>) -> Self {
        Self { client, name: name.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/projectvalidate/validProjectName".to_owned(),
        );

        config.query.push(("name".to_owned(), crate::core::QueryValue::Scalar(self.name.clone())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
