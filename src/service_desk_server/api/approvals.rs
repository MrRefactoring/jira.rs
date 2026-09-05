// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Approvals operations.
pub struct ApprovalsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ApprovalsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns an approval for a given approval ID.
    pub fn get_approval_by_id(
        &self,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> GetApprovalByIdRequest<'a> {
        GetApprovalByIdRequest::new(self.client, issue_id_or_key, approval_id)
    }

    /// Answer a pending approval.
    pub fn answer_approval(
        &self,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> AnswerApprovalRequest<'a> {
        AnswerApprovalRequest::new(self.client, issue_id_or_key, approval_id)
    }

    /// Returns approval comment config for a given approval ID.
    pub fn get_approval_comment_config(
        &self,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> GetApprovalCommentConfigRequest<'a> {
        GetApprovalCommentConfigRequest::new(self.client, issue_id_or_key, approval_id)
    }

    /// Returns all approvals on a request, for a given request Id/key.
    pub fn get_approvals(&self, issue_id_or_key: impl Into<String>) -> GetApprovalsRequest<'a> {
        GetApprovalsRequest::new(self.client, issue_id_or_key)
    }
}

/// Returns an approval for a given approval ID.
#[derive(Clone)]
pub struct GetApprovalByIdRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    approval_id: String,
}

impl<'a> GetApprovalByIdRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), approval_id: approval_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/approval/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.approval_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Approval> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Answer a pending approval.
#[derive(Clone)]
pub struct AnswerApprovalRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    approval_id: String,
    approval_decision_request: Option<ApprovalDecisionRequest>,
}

impl<'a> AnswerApprovalRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> Self {
        Self {
            client,
            issue_id_or_key: issue_id_or_key.into(),
            approval_id: approval_id.into(),
            approval_decision_request: None,
        }
    }

    #[must_use]
    pub fn approval_decision_request(mut self, value: ApprovalDecisionRequest) -> Self {
        self.approval_decision_request = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/approval/{}",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.approval_id)
            ),
        );

        let body = match serde_json::to_value(&self.approval_decision_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Approval> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns approval comment config for a given approval ID.
#[derive(Clone)]
pub struct GetApprovalCommentConfigRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    approval_id: String,
}

impl<'a> GetApprovalCommentConfigRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_id_or_key: impl Into<String>,
        approval_id: impl Into<String>,
    ) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), approval_id: approval_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/approval/{}/config",
                crate::core::encode_path_segment(&self.issue_id_or_key),
                crate::core::encode_path_segment(&self.approval_id)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ApprovalCommentConfig> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns all approvals on a request, for a given request Id/key.
#[derive(Clone)]
pub struct GetApprovalsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetApprovalsRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), start: None, limit: None }
    }

    /// The starting index of the returned objects. Base index: 0.
    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    /// The maximum number of items to return per page. Default: 50.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/servicedeskapi/request/{}/approval",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// Each page is asked for from where the one before it ended — from the offset already set on the request, or
    /// from the beginning — and the stream ends at the page that says it is the last, or at an empty one. Reading
    /// it needs `TryStreamExt` in scope, re-exported as [`crate::futures_util`] so no dependency of your own is
    /// required.
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Approval>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Approval>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
