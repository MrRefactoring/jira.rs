// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Webhooks operations.
pub struct WebhooksService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WebhooksService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the webhooks registered in this instance. Requires administrator permission.
    pub fn get_webhooks(&self) -> GetWebhooksRequest<'a> {
        GetWebhooksRequest::new(self.client)
    }

    /// Registers a webhook. Requires administrator permission.
    pub fn create_webhook(&self, webhook_input: WebhookInput) -> CreateWebhookRequest<'a> {
        CreateWebhookRequest::new(self.client, webhook_input)
    }

    /// Returns a registered webhook. Requires administrator permission.
    pub fn get_webhook(&self, webhook_id: i64) -> GetWebhookRequest<'a> {
        GetWebhookRequest::new(self.client, webhook_id)
    }

    /// Replaces a registered webhook. Requires administrator permission.
    pub fn update_webhook(&self, webhook_id: i64, webhook_input: WebhookInput) -> UpdateWebhookRequest<'a> {
        UpdateWebhookRequest::new(self.client, webhook_id, webhook_input)
    }

    /// Unregisters a webhook. Requires administrator permission.
    pub fn delete_webhook(&self, webhook_id: i64) -> DeleteWebhookRequest<'a> {
        DeleteWebhookRequest::new(self.client, webhook_id)
    }

    /// Returns how a webhook has been delivering. Requires administrator permission.
    pub fn get_webhook_statistics(&self, webhook_id: i64) -> GetWebhookStatisticsRequest<'a> {
        GetWebhookStatisticsRequest::new(self.client, webhook_id)
    }

    /// Returns the delivery statistics of a webhook, one entry per event it delivers. Requires administrator permission.
    pub fn get_webhook_statistics_summary(&self, webhook_id: i64) -> GetWebhookStatisticsSummaryRequest<'a> {
        GetWebhookStatisticsSummaryRequest::new(self.client, webhook_id)
    }

    /// Returns the transitions a webhook has been through. Requires administrator permission. The shape of an entry is not described here: an instance that has never delivered a webhook answers with an empty list, and guessing what a populated one holds would be worse than leaving it to the caller.
    pub fn get_webhook_transitions(&self, webhook_id: i64) -> GetWebhookTransitionsRequest<'a> {
        GetWebhookTransitionsRequest::new(self.client, webhook_id)
    }

    /// Returns the most recent delivery of a webhook. Requires administrator permission. Until the webhook has been delivered once Jira answers 204 and this resolves to `undefined`; the 204 is deliberately not declared, because declaring it is what makes the whole call type as `void` and hides the body that does arrive.
    pub fn get_latest_webhook_invocation(&self, webhook_id: i64) -> GetLatestWebhookInvocationRequest<'a> {
        GetLatestWebhookInvocationRequest::new(self.client, webhook_id)
    }
}

/// Returns the webhooks registered in this instance. Requires administrator permission.
pub struct GetWebhooksRequest<'a> {
    client: &'a crate::core::Client,
    event: Option<String>,
    statistics: Option<bool>,
    start: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetWebhooksRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, event: None, statistics: None, start: None, limit: None }
    }

    /// Only webhooks delivering this event.
    #[must_use]
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());

        self
    }

    /// Include delivery statistics with each webhook.
    #[must_use]
    pub fn statistics(mut self, value: bool) -> Self {
        self.statistics = Some(value);

        self
    }

    #[must_use]
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);

        self
    }

    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/jira-webhook/1.0/webhooks".to_owned());

        if let Some(value) = &self.event {
            config.query.push(("event".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.statistics {
            config.query.push(("statistics".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.start {
            config.query.push(("start".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Webhook>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Registers a webhook. Requires administrator permission.
pub struct CreateWebhookRequest<'a> {
    client: &'a crate::core::Client,
    webhook_input: WebhookInput,
}

impl<'a> CreateWebhookRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_input: WebhookInput) -> Self {
        Self { client, webhook_input }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/jira-webhook/1.0/webhooks".to_owned());

        let body = match serde_json::to_value(&self.webhook_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Webhook> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a registered webhook. Requires administrator permission.
pub struct GetWebhookRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> GetWebhookRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/jira-webhook/1.0/webhooks/{}", self.webhook_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Webhook> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Replaces a registered webhook. Requires administrator permission.
pub struct UpdateWebhookRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
    webhook_input: WebhookInput,
}

impl<'a> UpdateWebhookRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64, webhook_input: WebhookInput) -> Self {
        Self { client, webhook_id, webhook_input }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/jira-webhook/1.0/webhooks/{}", self.webhook_id),
        );

        let body = match serde_json::to_value(&self.webhook_input)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Webhook> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Unregisters a webhook. Requires administrator permission.
pub struct DeleteWebhookRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> DeleteWebhookRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/jira-webhook/1.0/webhooks/{}", self.webhook_id),
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

/// Returns how a webhook has been delivering. Requires administrator permission.
pub struct GetWebhookStatisticsRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> GetWebhookStatisticsRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/jira-webhook/1.0/webhooks/{}/statistics", self.webhook_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WebhookStatistics> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the delivery statistics of a webhook, one entry per event it delivers. Requires administrator permission.
pub struct GetWebhookStatisticsSummaryRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> GetWebhookStatisticsSummaryRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/jira-webhook/1.0/webhooks/{}/statistics/summary", self.webhook_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetWebhookStatisticsSummary> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the transitions a webhook has been through. Requires administrator permission. The shape of an entry is not described here: an instance that has never delivered a webhook answers with an empty list, and guessing what a populated one holds would be worse than leaving it to the caller.
pub struct GetWebhookTransitionsRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> GetWebhookTransitionsRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/jira-webhook/1.0/webhooks/{}/transitions", self.webhook_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the most recent delivery of a webhook. Requires administrator permission. Until the webhook has been delivered once Jira answers 204 and this resolves to `undefined`; the 204 is deliberately not declared, because declaring it is what makes the whole call type as `void` and hides the body that does arrive.
pub struct GetLatestWebhookInvocationRequest<'a> {
    client: &'a crate::core::Client,
    webhook_id: i64,
}

impl<'a> GetLatestWebhookInvocationRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_id: i64) -> Self {
        Self { client, webhook_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/jira-webhook/1.0/webhooks/{}/latest", self.webhook_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
