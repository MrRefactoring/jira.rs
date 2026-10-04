// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The JiraSettings operations.
pub struct JiraSettingsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> JiraSettingsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns an application property.
    pub fn get_application_property(&self) -> GetApplicationPropertyRequest<'a> {
        GetApplicationPropertyRequest::new(self.client)
    }

    /// Returns the properties that are displayed on the "General Configuration > Advanced Settings" page.
    pub fn get_advanced_settings(&self) -> GetAdvancedSettingsRequest<'a> {
        GetAdvancedSettingsRequest::new(self.client)
    }

    /// Update an application property via PUT. The "value" field present in the PUT will override the existing value.
    pub fn set_application_property(
        &self,
        id: impl Into<String>,
        body: ApplicationPropertyValue,
    ) -> SetApplicationPropertyRequest<'a> {
        SetApplicationPropertyRequest::new(self.client, id, body)
    }

    /// Returns the information if the optional features in Jira are enabled or disabled. If the time tracking is enabled, it also returns the detailed information about time tracking configuration.
    pub fn get_configuration(&self) -> JiraSettingsGetConfigurationRequest<'a> {
        JiraSettingsGetConfigurationRequest::new(self.client)
    }

    /// Sets the base URL that is configured for this Jira instance.
    pub fn set_base_url(&self) -> SetBaseURLRequest<'a> {
        SetBaseURLRequest::new(self.client)
    }
}

/// Returns an application property.
#[derive(Clone)]
pub struct GetApplicationPropertyRequest<'a> {
    client: &'a crate::core::Client,
    permission_level: Option<String>,
    key_filter: Option<String>,
    key: Option<String>,
}

impl<'a> GetApplicationPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, permission_level: None, key_filter: None, key: None }
    }

    /// when fetching a list specifies the permission level of all items in the list
    /// see [ApplicationPropertiesService.EditPermissionLevel](https://docs.atlassian.com/software/jira/docs/api/latest/com/atlassian/jira/bc/admin/ApplicationPropertiesService.EditPermissionLevel.html)
    #[must_use]
    pub fn permission_level(mut self, value: impl Into<String>) -> Self {
        self.permission_level = Some(value.into());

        self
    }

    /// when fetching a list allows the list to be filtered by the property's start of key
    /// e.g. "jira.lf.*" whould fetch only those permissions that are editable and whose keys start with
    ///      *                        "jira.lf.". This is a regex.
    #[must_use]
    pub fn key_filter(mut self, value: impl Into<String>) -> Self {
        self.key_filter = Some(value.into());

        self
    }

    /// a String containing the property key.
    #[must_use]
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/application-properties".to_owned());

        if let Some(value) = &self.permission_level {
            config.query.push(("permissionLevel".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.key_filter {
            config.query.push(("keyFilter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.key {
            config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ApplicationProperty>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the properties that are displayed on the "General Configuration > Advanced Settings" page.
#[derive(Clone)]
pub struct GetAdvancedSettingsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAdvancedSettingsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/application-properties/advanced-settings".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ApplicationProperty>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update an application property via PUT. The "value" field present in the PUT will override the existing value.
#[derive(Clone)]
pub struct SetApplicationPropertyRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
    body: ApplicationPropertyValue,
}

impl<'a> SetApplicationPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>, body: ApplicationPropertyValue) -> Self {
        Self { client, id: id.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/application-properties/{}", crate::core::encode_path_segment(&self.id)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ApplicationProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the information if the optional features in Jira are enabled or disabled. If the time tracking is enabled, it also returns the detailed information about time tracking configuration.
#[derive(Clone)]
pub struct JiraSettingsGetConfigurationRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> JiraSettingsGetConfigurationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/configuration".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Configuration> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the base URL that is configured for this Jira instance.
#[derive(Clone)]
pub struct SetBaseURLRequest<'a> {
    client: &'a crate::core::Client,
    body: Option<String>,
}

impl<'a> SetBaseURLRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, body: None }
    }

    #[must_use]
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/settings/baseUrl".to_owned());

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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
