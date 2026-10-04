// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueAttachments operations.
pub struct IssueAttachmentsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueAttachmentsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the meta information for an attachments, specifically if they are enabled and the maximum upload size allowed.
    pub fn get_attachment_meta(&self) -> GetAttachmentMetaRequest<'a> {
        GetAttachmentMetaRequest::new(self.client)
    }

    /// Returns the meta-data for an attachment, including the URI of the actual attached file.
    pub fn get_attachment(&self, id: impl Into<String>) -> GetAttachmentRequest<'a> {
        GetAttachmentRequest::new(self.client, id)
    }

    /// Remove an attachment from an issue.
    pub fn remove_attachment(&self, id: impl Into<String>) -> RemoveAttachmentRequest<'a> {
        RemoveAttachmentRequest::new(self.client, id)
    }

    /// Tries to expand an attachment. Output is human-readable and subject to change.
    pub fn expand_for_humans(&self, id: impl Into<String>) -> ExpandForHumansRequest<'a> {
        ExpandForHumansRequest::new(self.client, id)
    }

    /// Tries to expand an attachment. Output is raw and should be backwards-compatible through the course of time.
    pub fn expand_for_machines(&self, id: impl Into<String>) -> ExpandForMachinesRequest<'a> {
        ExpandForMachinesRequest::new(self.client, id)
    }

    /// Add one or more attachments to an issue.
    /// This resource expects a multipart post. The media-type multipart/form-data is defined in RFC 1867. Most client libraries have classes that make dealing with multipart posts simple. For instance, in Java the Apache HTTP Components library provides a MultiPartEntity that makes it simple to submit a multipart POST.
    /// In order to protect against XSRF attacks, because this method accepts multipart/form-data, it has XSRF protection
    /// on it. This means you must submit a header of X-Atlassian-Token: no-check with the request, otherwise it will be blocked.
    /// The name of the multipart/form-data parameter that contains attachments must be file.
    /// A simple example to upload a file called "myfile.txt" to issue TEST-123:
    /// curl -D- -u admin:admin -X POST -H "X-Atlassian-Token: no-check" -F "file=@myfile.txt" <http://myhost/rest/api/2/issue/TEST-123/attachments>
    pub fn add_attachment(
        &self,
        issue_id_or_key: impl Into<String>,
        attachments: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> AddAttachmentRequest<'a> {
        AddAttachmentRequest::new(self.client, issue_id_or_key, attachments)
    }
}

/// Returns the meta information for an attachments, specifically if they are enabled and the maximum upload size allowed.
#[derive(Clone)]
pub struct GetAttachmentMetaRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAttachmentMetaRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/attachment/meta".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AttachmentMeta> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the meta-data for an attachment, including the URI of the actual attached file.
#[derive(Clone)]
pub struct GetAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> GetAttachmentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/attachment/{}", crate::core::encode_path_segment(&self.id)),
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

/// Remove an attachment from an issue.
#[derive(Clone)]
pub struct RemoveAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> RemoveAttachmentRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/attachment/{}", crate::core::encode_path_segment(&self.id)),
        );

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

/// Tries to expand an attachment. Output is human-readable and subject to change.
#[derive(Clone)]
pub struct ExpandForHumansRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> ExpandForHumansRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/attachment/{}/expand/human", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<HumanReadableArchive> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Tries to expand an attachment. Output is raw and should be backwards-compatible through the course of time.
#[derive(Clone)]
pub struct ExpandForMachinesRequest<'a> {
    client: &'a crate::core::Client,
    id: String,
}

impl<'a> ExpandForMachinesRequest<'a> {
    fn new(client: &'a crate::core::Client, id: impl Into<String>) -> Self {
        Self { client, id: id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/attachment/{}/expand/raw", crate::core::encode_path_segment(&self.id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AttachmentArchiveImpl> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Add one or more attachments to an issue.
/// This resource expects a multipart post. The media-type multipart/form-data is defined in RFC 1867. Most client libraries have classes that make dealing with multipart posts simple. For instance, in Java the Apache HTTP Components library provides a MultiPartEntity that makes it simple to submit a multipart POST.
/// In order to protect against XSRF attacks, because this method accepts multipart/form-data, it has XSRF protection
/// on it. This means you must submit a header of X-Atlassian-Token: no-check with the request, otherwise it will be blocked.
/// The name of the multipart/form-data parameter that contains attachments must be file.
/// A simple example to upload a file called "myfile.txt" to issue TEST-123:
/// curl -D- -u admin:admin -X POST -H "X-Atlassian-Token: no-check" -F "file=@myfile.txt" <http://myhost/rest/api/2/issue/TEST-123/attachments>
#[derive(Clone)]
pub struct AddAttachmentRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    attachments: Vec<crate::core::Attachment>,
    content_type: Option<String>,
}

impl<'a> AddAttachmentRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        attachments: impl IntoIterator<Item = crate::core::Attachment>,
    ) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            attachments: attachments.into_iter().collect(),
            content_type: None,
        }
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
            format!("/rest/api/2/issue/{}/attachments", crate::core::encode_path_segment(&self.issue_id_or_key)),
        );

        config.headers.push(("X-Atlassian-Token".to_owned(), "no-check".to_owned()));

        config.body =
            Some(crate::core::Body::Multipart(crate::core::MultipartBody::new("file", self.attachments.clone())));

        config.content_type = self.content_type.clone().or(None);

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<AttachmentJson>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
