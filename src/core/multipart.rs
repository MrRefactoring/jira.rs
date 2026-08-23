use bytes::Bytes;

use crate::core::mime::mime_type_for;

/// One file in a multipart upload.
///
/// The content is held in memory rather than streamed: a request that is retried — after a 401, or after a 503 — has
/// to be sendable a second time, and a stream cannot be. Streaming uploads are a later addition, gated on the
/// endpoints that genuinely need them.
#[derive(Debug, Clone)]
pub struct Attachment {
    pub filename: String,
    pub content: Bytes,
    /// The content type to declare. Guessed from the filename when absent.
    pub content_type: Option<String>,
}

impl Attachment {
    pub fn new(filename: impl Into<String>, content: impl Into<Bytes>) -> Self {
        Attachment { filename: filename.into(), content: content.into(), content_type: None }
    }

    /// Declares a content type instead of guessing one from the filename.
    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Reads the file at `path` and names the attachment after it.
    pub async fn from_path(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let filename =
            path.file_name().map_or_else(|| "attachment".to_owned(), |name| name.to_string_lossy().into_owned());
        let content = tokio::fs::read(path).await?;

        Ok(Attachment::new(filename, content))
    }

    pub(crate) fn resolved_content_type(&self) -> String {
        self.content_type.clone().unwrap_or_else(|| mime_type_for(&self.filename).to_owned())
    }
}

/// A whole multipart body: the files, under the field name the endpoint reads them from.
#[derive(Debug, Clone)]
pub struct MultipartBody {
    /// The form field each file is appended under. `file` for Jira's attachment endpoints.
    pub field_name: String,
    pub attachments: Vec<Attachment>,
}

impl MultipartBody {
    pub fn new(field_name: impl Into<String>, attachments: Vec<Attachment>) -> Self {
        MultipartBody { field_name: field_name.into(), attachments }
    }

    /// A single file under the usual `file` field.
    pub fn file(attachment: Attachment) -> Self {
        MultipartBody::new("file", vec![attachment])
    }

    /// A whole set of files under the usual `file` field.
    pub fn files(attachments: Vec<Attachment>) -> Self {
        MultipartBody::new("file", attachments)
    }

    /// Builds the form afresh, so a retried request can send the same body again.
    pub(crate) fn to_form(&self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        for attachment in &self.attachments {
            let part = reqwest::multipart::Part::stream(attachment.content.clone())
                .file_name(attachment.filename.clone())
                .mime_str(&attachment.resolved_content_type())
                .unwrap_or_else(|_| {
                    reqwest::multipart::Part::stream(attachment.content.clone()).file_name(attachment.filename.clone())
                });

            form = form.part(self.field_name.clone(), part);
        }

        form
    }
}
