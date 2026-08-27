// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueSecuritySchemes operations.
pub struct IssueSecuritySchemesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueSecuritySchemesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all issue security schemes that are defined.
    pub fn get_issue_security_schemes(&self) -> GetIssueSecuritySchemesRequest<'a> {
        GetIssueSecuritySchemesRequest::new(self.client)
    }

    /// Returns the issue security scheme along with that are defined.
    pub fn get_issue_security_scheme(&self, id: impl Into<String>) -> GetIssueSecuritySchemeRequest<'a> {
        GetIssueSecuritySchemeRequest::new(self.client, id)
    }
}

/// Returns all issue security schemes that are defined.
pub struct GetIssueSecuritySchemesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueSecuritySchemesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/issuesecurityschemes".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SecuritySchemesJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the issue security scheme along with that are defined.
pub struct GetIssueSecuritySchemeRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetIssueSecuritySchemeRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/issuesecurityschemes/{}", self.id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SecuritySchemeJson> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
