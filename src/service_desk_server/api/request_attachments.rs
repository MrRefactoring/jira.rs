// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The RequestAttachments operations.
pub struct RequestAttachmentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> RequestAttachmentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Adds one or more temporary attachments that were created using [Attach temporary file](#servicedeskapi-servicedesk-{serviceDeskId}-attachTemporaryFile-post) to a customer request.
    ///
    /// The attachment visibility is set by the `public` field.
    ///
    /// Setting attachment visibility is dependent on the user's permission. For example, Agents can create either public or internal attachments, while Unlicensed users can only create internal attachments, and Customers can only create public attachments.
    ///
    /// An additional comment may be provided which will be prepended to the attachments.
    pub fn create_attachment(&self, issue_id_or_key: impl Into<String>) -> CreateAttachmentRequest<'a> {
        CreateAttachmentRequest::new(self.client, issue_id_or_key)
    }

    /// Create one or more temporary attachments, which can later be converted into permanent attachments on Create attachment.
    ///
    /// On successful execution, this resource will return a list of temporary attachment IDs, which are used in subsequent calls to convert the attachments into permanent attachments.
    ///
    /// This resource expects a multipart post. The media-type multipart/form-data is defined in RFC 1867. Most client libraries have classes that make dealing with multipart posts simple. For instance, in Java the Apache HTTP Components library provides a MultiPartEntity that makes it simple to submit a multipart POST.
    ///
    /// In order to protect against XSRF attacks, because this method accepts multipart/form-data, it has XSRF protection on it. This means you must submit a header of X-Atlassian-Token: no-check with the request, otherwise it will be blocked.
    ///
    /// The name of the multipart/form-data parameter that contains attachments must be "file".
    ///
    /// A simple example to upload a file called "myfile.txt" in service project with ID 10001
    ///
    /// `curl -D- -u customer:customer -X POST -H "X-Atlassian-Token: no-check" -F`.
    pub fn attach_temporary_file(
        &self,
        service_desk_id: impl Into<String>,
        body: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> AttachTemporaryFileRequest<'a> {
        AttachTemporaryFileRequest::new(self.client, service_desk_id, body)
    }
}

/// Adds one or more temporary attachments that were created using [Attach temporary file](#servicedeskapi-servicedesk-{serviceDeskId}-attachTemporaryFile-post) to a customer request.
///
/// The attachment visibility is set by the `public` field.
///
/// Setting attachment visibility is dependent on the user's permission. For example, Agents can create either public or internal attachments, while Unlicensed users can only create internal attachments, and Customers can only create public attachments.
///
/// An additional comment may be provided which will be prepended to the attachments.
pub struct CreateAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    attachment_create: Option<AttachmentCreate>,
}

impl<'a> CreateAttachmentRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), attachment_create: None }
    }

    #[must_use]
    pub fn attachment_create(mut self, value: AttachmentCreate) -> Self {
        self.attachment_create = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/attachment",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        let body = match serde_json::to_value(&self.attachment_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AttachmentCreateResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Create one or more temporary attachments, which can later be converted into permanent attachments on Create attachment.
///
/// On successful execution, this resource will return a list of temporary attachment IDs, which are used in subsequent calls to convert the attachments into permanent attachments.
///
/// This resource expects a multipart post. The media-type multipart/form-data is defined in RFC 1867. Most client libraries have classes that make dealing with multipart posts simple. For instance, in Java the Apache HTTP Components library provides a MultiPartEntity that makes it simple to submit a multipart POST.
///
/// In order to protect against XSRF attacks, because this method accepts multipart/form-data, it has XSRF protection on it. This means you must submit a header of X-Atlassian-Token: no-check with the request, otherwise it will be blocked.
///
/// The name of the multipart/form-data parameter that contains attachments must be "file".
///
/// A simple example to upload a file called "myfile.txt" in service project with ID 10001
///
/// `curl -D- -u customer:customer -X POST -H "X-Atlassian-Token: no-check" -F`.
pub struct AttachTemporaryFileRequest<'a> {
    client: &'a crate::core::Client,
    service_desk_id: String,
    body: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> AttachTemporaryFileRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        service_desk_id: impl Into<String>,
        body: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self { client, service_desk_id: service_desk_id.into(), body: body.into_iter().collect(), content_type: None }
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
            format!(
                "/rest/servicedeskapi/servicedesk/{}/attachTemporaryFile",
                crate::core::encode_path_segment(&self.service_desk_id)
            ),
        );

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body = Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("file", self.body.clone())));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CreateTemporaryWebAttachmentResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
