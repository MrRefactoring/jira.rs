// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Attachments operations.
pub struct AttachmentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AttachmentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Get the attachments for an object by object ID.
    pub fn get_attachments(&self, object_id: impl Into<String>) -> GetAttachmentsRequest<'a> {
        GetAttachmentsRequest::new(self.client, object_id)
    }

    /// Add an attachment to an object by object ID.
    pub fn add_attachments(
        &self,
        object_id: impl Into<String>,
        attachments: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> AddAttachmentsRequest<'a> {
        AddAttachmentsRequest::new(self.client, object_id, attachments)
    }

    /// Delete an attachment by attachment ID.
    pub fn delete_attachment(&self, attachment_id: impl Into<String>) -> DeleteAttachmentRequest<'a> {
        DeleteAttachmentRequest::new(self.client, attachment_id)
    }
}

/// Get the attachments for an object by object ID.
pub struct GetAttachmentsRequest<'a> {
    client: &'a crate::core::Client,
    object_id: String,
}

impl<'a> GetAttachmentsRequest<'a> {
    fn new(client: &'a crate::core::Client, object_id: impl Into<String>) -> Self {
        Self { client, object_id: object_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/assets/1.0/attachments/object/{}", crate::core::encode_path_segment(&self.object_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Attachment>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Add an attachment to an object by object ID.
pub struct AddAttachmentsRequest<'a> {
    client: &'a crate::core::Client,
    object_id: String,
    attachments: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> AddAttachmentsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        object_id: impl Into<String>,
        attachments: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self { client, object_id: object_id.into(), attachments: attachments.into_iter().collect(), content_type: None }
    }

    /// The media type of the bytes being sent, e.g. `image/png`.
    #[must_use]
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/assets/1.0/attachments/object/{}", crate::core::encode_path_segment(&self.object_id)),
        );

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body =
            Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("file", self.attachments.clone())));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<UploadedAttachment>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Delete an attachment by attachment ID.
pub struct DeleteAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    attachment_id: String,
}

impl<'a> DeleteAttachmentRequest<'a> {
    fn new(client: &'a crate::core::Client, attachment_id: impl Into<String>) -> Self {
        Self { client, attachment_id: attachment_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/assets/1.0/attachments/{}", crate::core::encode_path_segment(&self.attachment_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Attachment> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
