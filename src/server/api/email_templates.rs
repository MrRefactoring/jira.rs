// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The EmailTemplates operations.
pub struct EmailTemplatesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> EmailTemplatesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a zip file containing email templates at local home and returns the file.
    pub fn download_email_templates(&self) -> DownloadEmailTemplatesRequest<'a> {
        DownloadEmailTemplatesRequest::new(self.client)
    }

    /// Extracts given zip file to temporary templates folder. If the folder already exists it will replace it's content
    pub fn upload_email_templates(&self, body: impl Into<bytes::Bytes>) -> UploadEmailTemplatesRequest<'a> {
        UploadEmailTemplatesRequest::new(self.client, body)
    }

    /// Replaces the current email templates pack with previously uploaded one, if exists.
    pub fn apply_email_templates(&self) -> ApplyEmailTemplatesRequest<'a> {
        ApplyEmailTemplatesRequest::new(self.client)
    }

    /// Replaces the current email templates pack with default templates, which are copied over from Jira binaries.
    pub fn revert_email_templates_to_default(&self) -> RevertEmailTemplatesToDefaultRequest<'a> {
        RevertEmailTemplatesToDefaultRequest::new(self.client)
    }

    /// Returns a list of root templates mapped with Event Types. The list can be used to decide which test emails to send.
    pub fn get_email_types(&self) -> GetEmailTypesRequest<'a> {
        GetEmailTypesRequest::new(self.client)
    }
}

/// Creates a zip file containing email templates at local home and returns the file.
#[derive(Clone)]
pub struct DownloadEmailTemplatesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> DownloadEmailTemplatesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/email-templates".to_owned());

        config.headers.push(("Accept".to_owned(), "application/zip".to_owned()));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<bytes::Bytes> {
        self.client.send_bytes(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Extracts given zip file to temporary templates folder. If the folder already exists it will replace it's content
#[derive(Clone)]
pub struct UploadEmailTemplatesRequest<'a> {
    client: &'a crate::core::Client,
    body: bytes::Bytes,
    content_type: Option<String>,
}

impl<'a> UploadEmailTemplatesRequest<'a> {
    fn new(client: &'a crate::core::Client, body: impl Into<bytes::Bytes>) -> Self {
        Self { client, body: body.into(), content_type: None }
    }

    /// The media type of the bytes being sent, e.g. `image/png`.
    #[must_use]
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/email-templates".to_owned());

        config.body = Some(crate::core::Body::Bytes(self.body.clone()));

        config.content_type = self.content_type.clone().or(Some("application/zip".to_owned()));

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

/// Replaces the current email templates pack with previously uploaded one, if exists.
#[derive(Clone)]
pub struct ApplyEmailTemplatesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ApplyEmailTemplatesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/email-templates/apply".to_owned());

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

/// Replaces the current email templates pack with default templates, which are copied over from Jira binaries.
#[derive(Clone)]
pub struct RevertEmailTemplatesToDefaultRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> RevertEmailTemplatesToDefaultRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/email-templates/revert".to_owned());

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

/// Returns a list of root templates mapped with Event Types. The list can be used to decide which test emails to send.
#[derive(Clone)]
pub struct GetEmailTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetEmailTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/email-templates/types".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EmailTemplateTypes> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
