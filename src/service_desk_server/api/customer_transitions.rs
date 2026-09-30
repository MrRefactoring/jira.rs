// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The CustomerTransitions operations.
pub struct CustomerTransitionsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> CustomerTransitionsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of transitions that customers can perform on the request.
    ///
    ///  **Permissions:**
    ///
    ///  The calling user must be able to view the request.
    pub fn get_customer_transitions(&self, issue_id_or_key: impl Into<String>) -> GetCustomerTransitionsRequest<'a> {
        GetCustomerTransitionsRequest::new(self.client, issue_id_or_key)
    }

    /// Perform a customer transition for a given request and transition ID.  An optional comment can be included to provide a reason for the transition.**Permissions:**
    ///  The calling user must be able to view the request and have the Transition Issues permission.If an additional comment is passed the calling user must also have the Add Comments permission.
    pub fn perform_customer_transition(
        &self,
        issue_id_or_key: impl Into<String>,
    ) -> PerformCustomerTransitionRequest<'a> {
        PerformCustomerTransitionRequest::new(self.client, issue_id_or_key)
    }
}

/// Returns a list of transitions that customers can perform on the request.
///
///  **Permissions:**
///
///  The calling user must be able to view the request.
#[derive(Clone)]
pub struct GetCustomerTransitionsRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetCustomerTransitionsRequest<'a> {
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
                "/rest/servicedeskapi/request/{}/transition",
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
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<CustomerTransition>> {
        let first = self.start.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, offset| {
            request.start = Some(offset);

            request.send()
        })
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomerTransition>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Perform a customer transition for a given request and transition ID.  An optional comment can be included to provide a reason for the transition.**Permissions:**
///  The calling user must be able to view the request and have the Transition Issues permission.If an additional comment is passed the calling user must also have the Add Comments permission.
#[derive(Clone)]
pub struct PerformCustomerTransitionRequest<'a> {
    client: &'a crate::core::Client,
    issue_id_or_key: String,
    customer_transition_execution: Option<CustomerTransitionExecution>,
}

impl<'a> PerformCustomerTransitionRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_id_or_key: impl Into<String>) -> Self {
        Self { client, issue_id_or_key: issue_id_or_key.into(), customer_transition_execution: None }
    }

    #[must_use]
    pub fn customer_transition_execution(mut self, value: CustomerTransitionExecution) -> Self {
        self.customer_transition_execution = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/servicedeskapi/request/{}/transition",
                crate::core::encode_path_segment(&self.issue_id_or_key)
            ),
        );

        let body = match serde_json::to_value(&self.customer_transition_execution)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

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
