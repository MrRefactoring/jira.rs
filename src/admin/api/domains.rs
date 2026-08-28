// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Domains operations.
pub struct DomainsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DomainsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of domains in an organization one page at a time.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:domains:admin`
    pub fn get_domains(&self, org_id: impl Into<String>) -> GetDomainsRequest<'a> {
        GetDomainsRequest::new(self.client, org_id)
    }

    /// Returns information about a single verified domain by ID.
    ///
    /// #### Scopes
    /// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:domains:admin`
    pub fn get_domain_by_id(
        &self,
        org_id: impl Into<String>,
        domain_id: impl Into<String>,
    ) -> GetDomainByIdRequest<'a> {
        GetDomainByIdRequest::new(self.client, org_id, domain_id)
    }
}

/// Returns a list of domains in an organization one page at a time.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:domains:admin`
pub struct GetDomainsRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    cursor: Option<String>,
}

impl<'a> GetDomainsRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), cursor: None }
    }

    /// Sets the starting point for the page of results to return.
    #[must_use]
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/admin/v1/orgs/{}/domains", crate::core::encode_path_segment(&self.org_id)),
        );

        if let Some(value) = &self.cursor {
            config.query.push(("cursor".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<DomainPage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns information about a single verified domain by ID.
///
/// #### Scopes
/// **[Authorization scopes](https://developer.atlassian.com/cloud/admin/scopes/) required:** `read:domains:admin`
pub struct GetDomainByIdRequest<'a> {
    client: &'a crate::core::Client,
    org_id: String,
    domain_id: String,
}

impl<'a> GetDomainByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, org_id: impl Into<String>, domain_id: impl Into<String>) -> Self {
        Self { client, org_id: org_id.into(), domain_id: domain_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/admin/v1/orgs/{}/domains/{}",
                crate::core::encode_path_segment(&self.org_id),
                crate::core::encode_path_segment(&self.domain_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Domain> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
