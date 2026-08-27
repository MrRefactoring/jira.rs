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

    /// Returns a [paginated](#pagination) list of the webhooks registered by the calling app.
    ///
    /// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
    pub fn get_dynamic_webhooks_for_app(&self) -> GetDynamicWebhooksForAppRequest<'a> {
        GetDynamicWebhooksForAppRequest::new(self.client)
    }

    /// Registers webhooks.
    ///
    /// **NOTE:** for non-public OAuth apps, webhooks are delivered only if there is a match between the app owner and the user who registered a dynamic webhook.
    ///
    /// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
    pub fn register_dynamic_webhooks(
        &self,
        webhook_registration_details: WebhookRegistrationDetails,
    ) -> RegisterDynamicWebhooksRequest<'a> {
        RegisterDynamicWebhooksRequest::new(self.client, webhook_registration_details)
    }

    /// Removes webhooks by ID. Only webhooks registered by the calling app are removed. If webhooks created by other apps are specified, they are ignored.
    ///
    /// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
    pub fn delete_webhook_by_id(
        &self,
        container_for_webhook_i_ds: ContainerForWebhookIDs,
    ) -> DeleteWebhookByIdRequest<'a> {
        DeleteWebhookByIdRequest::new(self.client, container_for_webhook_i_ds)
    }

    /// Extends the life of webhook. Webhooks registered through the REST API expire after 30 days. Call this operation to keep them alive.
    ///
    /// Unrecognized webhook IDs (those that are not found or belong to other apps) are ignored.
    ///
    /// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
    pub fn refresh_webhooks(&self, container_for_webhook_i_ds: ContainerForWebhookIDs) -> RefreshWebhooksRequest<'a> {
        RefreshWebhooksRequest::new(self.client, container_for_webhook_i_ds)
    }
}

/// Returns a [paginated](#pagination) list of the webhooks registered by the calling app.
///
/// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
pub struct GetDynamicWebhooksForAppRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetDynamicWebhooksForAppRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, start_at: None, max_results: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/webhook".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<Webhook>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Registers webhooks.
///
/// **NOTE:** for non-public OAuth apps, webhooks are delivered only if there is a match between the app owner and the user who registered a dynamic webhook.
///
/// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
pub struct RegisterDynamicWebhooksRequest<'a> {
    client: &'a crate::core::Client,
    webhook_registration_details: WebhookRegistrationDetails,
}

impl<'a> RegisterDynamicWebhooksRequest<'a> {
    fn new(client: &'a crate::core::Client, webhook_registration_details: WebhookRegistrationDetails) -> Self {
        Self { client, webhook_registration_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/webhook".to_owned());

        let body = match serde_json::to_value(&self.webhook_registration_details)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ContainerForRegisteredWebhooks> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Removes webhooks by ID. Only webhooks registered by the calling app are removed. If webhooks created by other apps are specified, they are ignored.
///
/// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
pub struct DeleteWebhookByIdRequest<'a> {
    client: &'a crate::core::Client,
    container_for_webhook_i_ds: ContainerForWebhookIDs,
}

impl<'a> DeleteWebhookByIdRequest<'a> {
    fn new(client: &'a crate::core::Client, container_for_webhook_i_ds: ContainerForWebhookIDs) -> Self {
        Self { client, container_for_webhook_i_ds }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/3/webhook".to_owned());

        let body = match serde_json::to_value(&self.container_for_webhook_i_ds)? {
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

/// Extends the life of webhook. Webhooks registered through the REST API expire after 30 days. Call this operation to keep them alive.
///
/// Unrecognized webhook IDs (those that are not found or belong to other apps) are ignored.
///
/// **[Permissions](#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/#connect-apps) and [OAuth 2.0](https://developer.atlassian.com/cloud/jira/platform/oauth-2-3lo-apps) apps can use this operation.
pub struct RefreshWebhooksRequest<'a> {
    client: &'a crate::core::Client,
    container_for_webhook_i_ds: ContainerForWebhookIDs,
}

impl<'a> RefreshWebhooksRequest<'a> {
    fn new(client: &'a crate::core::Client, container_for_webhook_i_ds: ContainerForWebhookIDs) -> Self {
        Self { client, container_for_webhook_i_ds }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/webhook/refresh".to_owned());

        let body = match serde_json::to_value(&self.container_for_webhook_i_ds)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WebhooksExpirationDate> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
