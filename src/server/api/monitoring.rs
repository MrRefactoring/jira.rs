// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Monitoring operations.
pub struct MonitoringService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> MonitoringService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Checks if App Monitoring is enabled
    pub fn is_app_monitoring_enabled(&self) -> IsAppMonitoringEnabledRequest<'a> {
        IsAppMonitoringEnabledRequest::new(self.client)
    }

    /// Enables or disables App Monitoring
    pub fn set_app_monitoring_enabled(
        &self,
        app_monitoring_rest_entity: AppMonitoringRestEntity,
    ) -> SetAppMonitoringEnabledRequest<'a> {
        SetAppMonitoringEnabledRequest::new(self.client, app_monitoring_rest_entity)
    }

    /// Checks if IPD Monitoring is enabled
    pub fn is_ipd_monitoring_enabled(&self) -> IsIpdMonitoringEnabledRequest<'a> {
        IsIpdMonitoringEnabledRequest::new(self.client)
    }

    /// Enables or disables IPD Monitoring
    pub fn set_ipd_monitoring_enabled(
        &self,
        ipd_monitoring_rest_entity: IpdMonitoringRestEntity,
    ) -> SetIpdMonitoringEnabledRequest<'a> {
        SetIpdMonitoringEnabledRequest::new(self.client, ipd_monitoring_rest_entity)
    }

    /// Checks if JMX metrics are being exposed
    pub fn are_metrics_exposed(&self) -> AreMetricsExposedRequest<'a> {
        AreMetricsExposedRequest::new(self.client)
    }

    /// Gets the available JMX metrics
    pub fn get_available_metrics(&self) -> GetAvailableMetricsRequest<'a> {
        GetAvailableMetricsRequest::new(self.client)
    }

    /// Starts exposing JMX metrics
    pub fn start(&self) -> StartRequest<'a> {
        StartRequest::new(self.client)
    }

    /// Stops exposing JMX metrics
    pub fn stop(&self) -> StopRequest<'a> {
        StopRequest::new(self.client)
    }
}

/// Checks if App Monitoring is enabled
#[derive(Clone)]
pub struct IsAppMonitoringEnabledRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IsAppMonitoringEnabledRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/monitoring/app".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AppMonitoringRestEntity> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Enables or disables App Monitoring
#[derive(Clone)]
pub struct SetAppMonitoringEnabledRequest<'a> {
    client: &'a crate::core::Client,
    app_monitoring_rest_entity: AppMonitoringRestEntity,
}

impl<'a> SetAppMonitoringEnabledRequest<'a> {
    fn new(client: &'a crate::core::Client, app_monitoring_rest_entity: AppMonitoringRestEntity) -> Self {
        Self { client, app_monitoring_rest_entity }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/monitoring/app".to_owned());

        let body = match serde_json::to_value(&self.app_monitoring_rest_entity)? {
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

/// Checks if IPD Monitoring is enabled
#[derive(Clone)]
pub struct IsIpdMonitoringEnabledRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IsIpdMonitoringEnabledRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/monitoring/ipd".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IpdMonitoringRestEntity> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Enables or disables IPD Monitoring
#[derive(Clone)]
pub struct SetIpdMonitoringEnabledRequest<'a> {
    client: &'a crate::core::Client,
    ipd_monitoring_rest_entity: IpdMonitoringRestEntity,
}

impl<'a> SetIpdMonitoringEnabledRequest<'a> {
    fn new(client: &'a crate::core::Client, ipd_monitoring_rest_entity: IpdMonitoringRestEntity) -> Self {
        Self { client, ipd_monitoring_rest_entity }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/monitoring/ipd".to_owned());

        let body = match serde_json::to_value(&self.ipd_monitoring_rest_entity)? {
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

/// Checks if JMX metrics are being exposed
#[derive(Clone)]
pub struct AreMetricsExposedRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AreMetricsExposedRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/monitoring/jmx/areMetricsExposed".to_owned(),
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

/// Gets the available JMX metrics
#[derive(Clone)]
pub struct GetAvailableMetricsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAvailableMetricsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/2/monitoring/jmx/getAvailableMetrics".to_owned(),
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

/// Starts exposing JMX metrics
#[derive(Clone)]
pub struct StartRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> StartRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/monitoring/jmx/startExposing".to_owned(),
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

/// Stops exposing JMX metrics
#[derive(Clone)]
pub struct StopRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> StopRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/monitoring/jmx/stopExposing".to_owned(),
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
