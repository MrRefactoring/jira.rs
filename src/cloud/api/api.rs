// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Api operations.
pub struct ApiService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ApiService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns worklog details for a list of issue ID and worklog ID pairs.
    ///
    /// This is an internal API for bulk fetching worklogs by their issue and worklog IDs. Worklogs that don't exist will be filtered out from the response.
    ///
    /// The returned list of worklogs is limited to 1000 items.
    ///
    /// **[Permissions](#permissions) required:** This is an internal service-to-service API that requires ASAP authentication. No user permission checks are performed as this bypasses normal user context.
    pub fn get_worklogs_by_issue_id_and_worklog_id(
        &self,
        bulk_worklog_key_request: BulkWorklogKeyRequest,
    ) -> GetWorklogsByIssueIdAndWorklogIdRequest<'a> {
        GetWorklogsByIssueIdAndWorklogIdRequest::new(self.client, bulk_worklog_key_request)
    }
}

/// Returns worklog details for a list of issue ID and worklog ID pairs.
///
/// This is an internal API for bulk fetching worklogs by their issue and worklog IDs. Worklogs that don't exist will be filtered out from the response.
///
/// The returned list of worklogs is limited to 1000 items.
///
/// **[Permissions](#permissions) required:** This is an internal service-to-service API that requires ASAP authentication. No user permission checks are performed as this bypasses normal user context.
pub struct GetWorklogsByIssueIdAndWorklogIdRequest<'a> {
    client: &'a crate::core::Client,
    bulk_worklog_key_request: BulkWorklogKeyRequest,
}

impl<'a> GetWorklogsByIssueIdAndWorklogIdRequest<'a> {
    fn new(client: &'a crate::core::Client, bulk_worklog_key_request: BulkWorklogKeyRequest) -> Self {
        Self { client, bulk_worklog_key_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/internal/api/latest/worklog/bulk".to_owned(),
        );

        let body = match serde_json::to_value(&self.bulk_worklog_key_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkWorklogKeyResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
