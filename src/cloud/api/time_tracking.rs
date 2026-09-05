// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The TimeTracking operations.
pub struct TimeTrackingService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> TimeTrackingService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the time tracking provider that is currently selected. Note that if time tracking is disabled, then a successful but empty response is returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_selected_time_tracking_implementation(&self) -> GetSelectedTimeTrackingImplementationRequest<'a> {
        GetSelectedTimeTrackingImplementationRequest::new(self.client)
    }

    /// Selects a time tracking provider.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn select_time_tracking_implementation(
        &self,
        time_tracking_provider: TimeTrackingProvider,
    ) -> SelectTimeTrackingImplementationRequest<'a> {
        SelectTimeTrackingImplementationRequest::new(self.client, time_tracking_provider)
    }

    /// Returns all time tracking providers. By default, Jira only has one time tracking provider: *JIRA provided time tracking*. However, you can install other time tracking providers via apps from the Atlassian Marketplace. For more information on time tracking providers, see the documentation for the [ Time Tracking Provider](https://developer.atlassian.com/cloud/jira/platform/modules/time-tracking-provider/) module.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_available_time_tracking_implementations(&self) -> GetAvailableTimeTrackingImplementationsRequest<'a> {
        GetAvailableTimeTrackingImplementationsRequest::new(self.client)
    }

    /// Returns the time tracking settings. This includes settings such as the time format, default time unit, and others. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_shared_time_tracking_configuration(&self) -> GetSharedTimeTrackingConfigurationRequest<'a> {
        GetSharedTimeTrackingConfigurationRequest::new(self.client)
    }

    /// Sets the time tracking settings.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn set_shared_time_tracking_configuration(
        &self,
        time_tracking_configuration: TimeTrackingConfiguration,
    ) -> SetSharedTimeTrackingConfigurationRequest<'a> {
        SetSharedTimeTrackingConfigurationRequest::new(self.client, time_tracking_configuration)
    }
}

/// Returns the time tracking provider that is currently selected. Note that if time tracking is disabled, then a successful but empty response is returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetSelectedTimeTrackingImplementationRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetSelectedTimeTrackingImplementationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/configuration/timetracking".to_owned(),
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

/// Selects a time tracking provider.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct SelectTimeTrackingImplementationRequest<'a> {
    client: &'a crate::core::Client,
    time_tracking_provider: TimeTrackingProvider,
}

impl<'a> SelectTimeTrackingImplementationRequest<'a> {
    fn new(client: &'a crate::core::Client, time_tracking_provider: TimeTrackingProvider) -> Self {
        Self { client, time_tracking_provider }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/api/3/configuration/timetracking".to_owned(),
        );

        let body = match serde_json::to_value(&self.time_tracking_provider)? {
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

/// Returns all time tracking providers. By default, Jira only has one time tracking provider: *JIRA provided time tracking*. However, you can install other time tracking providers via apps from the Atlassian Marketplace. For more information on time tracking providers, see the documentation for the [ Time Tracking Provider](https://developer.atlassian.com/cloud/jira/platform/modules/time-tracking-provider/) module.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetAvailableTimeTrackingImplementationsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAvailableTimeTrackingImplementationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/configuration/timetracking/list".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<TimeTrackingProvider>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the time tracking settings. This includes settings such as the time format, default time unit, and others. For more information, see [Configuring time tracking](https://confluence.atlassian.com/x/qoXKM).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct GetSharedTimeTrackingConfigurationRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetSharedTimeTrackingConfigurationRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/configuration/timetracking/options".to_owned(),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TimeTrackingConfiguration> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the time tracking settings.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
#[derive(Clone)]
pub struct SetSharedTimeTrackingConfigurationRequest<'a> {
    client: &'a crate::core::Client,
    time_tracking_configuration: TimeTrackingConfiguration,
}

impl<'a> SetSharedTimeTrackingConfigurationRequest<'a> {
    fn new(client: &'a crate::core::Client, time_tracking_configuration: TimeTrackingConfiguration) -> Self {
        Self { client, time_tracking_configuration }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/api/3/configuration/timetracking/options".to_owned(),
        );

        let body = match serde_json::to_value(&self.time_tracking_configuration)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TimeTrackingConfiguration> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
