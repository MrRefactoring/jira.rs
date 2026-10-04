// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetIssueNavigatorDefaultColumnsRequestColumnRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
}

/// The IssueNavigatorSettings operations.
pub struct IssueNavigatorSettingsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueNavigatorSettingsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the default system columns for issue navigator. Admin permission will be required.
    pub fn get_issue_navigator_default_columns(&self) -> GetIssueNavigatorDefaultColumnsRequest<'a> {
        GetIssueNavigatorDefaultColumnsRequest::new(self.client)
    }

    /// Sets the default system columns for issue navigator. Admin permission will be required.
    pub fn set_issue_navigator_default_columns(
        &self,
        column_request_body: SetIssueNavigatorDefaultColumnsRequestColumnRequestBody,
    ) -> SetIssueNavigatorDefaultColumnsRequest<'a> {
        SetIssueNavigatorDefaultColumnsRequest::new(self.client, column_request_body)
    }
}

/// Returns the default system columns for issue navigator. Admin permission will be required.
#[derive(Clone)]
pub struct GetIssueNavigatorDefaultColumnsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueNavigatorDefaultColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/settings/columns".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnOptions>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default system columns for issue navigator. Admin permission will be required.
#[derive(Clone)]
pub struct SetIssueNavigatorDefaultColumnsRequest<'a> {
    client: &'a crate::core::Client,
    column_request_body: SetIssueNavigatorDefaultColumnsRequestColumnRequestBody,
}

impl<'a> SetIssueNavigatorDefaultColumnsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        column_request_body: SetIssueNavigatorDefaultColumnsRequestColumnRequestBody,
    ) -> Self {
        Self { client, column_request_body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/settings/columns".to_owned());

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.column_request_body)?));

        config.content_type = Some("application/x-www-form-urlencoded".to_owned());

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
