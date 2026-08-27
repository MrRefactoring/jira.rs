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

    /// Sets the base URL that is configured for this Jira instance.
    pub fn set_base_url(&self) -> SetBaseURLRequest<'a> {
        SetBaseURLRequest::new(self.client)
    }

    /// Returns the default system columns for issue navigator. Admin permission will be required.
    pub fn get_issue_navigator_default_columns(&self) -> GetIssueNavigatorDefaultColumnsRequest<'a> {
        GetIssueNavigatorDefaultColumnsRequest::new(self.client)
    }

    /// Sets the default system columns for issue navigator. Admin permission will be required.
    pub fn set_issue_navigator_default_columns_form(&self) -> SetIssueNavigatorDefaultColumnsFormRequest<'a> {
        SetIssueNavigatorDefaultColumnsFormRequest::new(self.client)
    }
}

/// Sets the base URL that is configured for this Jira instance.
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

/// Returns the default system columns for issue navigator. Admin permission will be required.
pub struct GetIssueNavigatorDefaultColumnsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueNavigatorDefaultColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/settings/columns".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnOptions>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default system columns for issue navigator. Admin permission will be required.
pub struct SetIssueNavigatorDefaultColumnsFormRequest<'a> {
    client: &'a crate::core::Client,
    columns: Option<Vec<String>>,
}

impl<'a> SetIssueNavigatorDefaultColumnsFormRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, columns: None }
    }

    #[must_use]
    pub fn columns(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.columns = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/settings/columns".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.columns {
            body.insert("columns".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        config.content_type = Some("application/x-www-form-urlencoded".to_owned());

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
