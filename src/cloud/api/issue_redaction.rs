// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueRedaction operations.
pub struct IssueRedactionService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueRedactionService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Submit a job to redact issue field data. This will trigger the redaction of the data in the specified fields asynchronously.
    ///
    /// The redaction status can be polled using the job id.
    pub fn redact(&self, bulk_redaction_request: BulkRedactionRequest) -> RedactRequest<'a> {
        RedactRequest::new(self.client, bulk_redaction_request)
    }

    /// Retrieves the current status of a redaction job ID.
    ///
    /// The jobStatus will be one of the following:
    ///
    ///  *  IN\_PROGRESS - The redaction job is currently in progress
    ///  *  COMPLETED - The redaction job has completed successfully.
    ///  *  PENDING - The redaction job has not started yet
    pub fn get_redaction_status(&self, job_id: impl Into<String>) -> GetRedactionStatusRequest<'a> {
        GetRedactionStatusRequest::new(self.client, job_id)
    }
}

/// Submit a job to redact issue field data. This will trigger the redaction of the data in the specified fields asynchronously.
///
/// The redaction status can be polled using the job id.
#[derive(Clone)]
pub struct RedactRequest<'a> {
    client: &'a crate::core::Client,
    bulk_redaction_request: BulkRedactionRequest,
}

impl<'a> RedactRequest<'a> {
    fn new(client: &'a crate::core::Client, bulk_redaction_request: BulkRedactionRequest) -> Self {
        Self { client, bulk_redaction_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/redact".to_owned());

        let body = match serde_json::to_value(&self.bulk_redaction_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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

/// Retrieves the current status of a redaction job ID.
///
/// The jobStatus will be one of the following:
///
///  *  IN\_PROGRESS - The redaction job is currently in progress
///  *  COMPLETED - The redaction job has completed successfully.
///  *  PENDING - The redaction job has not started yet
#[derive(Clone)]
pub struct GetRedactionStatusRequest<'a> {
    client: &'a crate::core::Client,
    job_id: String,
}

impl<'a> GetRedactionStatusRequest<'a> {
    fn new(client: &'a crate::core::Client, job_id: impl Into<String>) -> Self {
        Self { client, job_id: job_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/redact/status/{}", crate::core::encode_path_segment(&self.job_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<RedactionJobStatusResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
