// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueNavigatorSettings operations.
pub struct IssueNavigatorSettingsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueNavigatorSettingsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the default issue navigator columns.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_issue_navigator_default_columns(&self) -> GetIssueNavigatorDefaultColumnsRequest<'a> {
        GetIssueNavigatorDefaultColumnsRequest::new(self.client)
    }

    /// Sets the default issue navigator columns.
    ///
    /// The `columns` parameter accepts a navigable field value and is expressed as HTML form data. To specify multiple columns, pass multiple `columns` parameters. For example, in curl:
    ///
    /// `curl -X PUT -d columns=summary -d columns=description https://your-domain.atlassian.net/rest/api/3/settings/columns`
    ///
    /// If no column details are sent, then all default columns are removed.
    ///
    /// A navigable field is one that can be used as a column on the issue navigator. Find details of navigable issue columns using [Get fields](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-fields/#api-rest-api-3-field-get).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn set_issue_navigator_default_columns(
        &self,
        column_request_body: ColumnRequestBody,
    ) -> SetIssueNavigatorDefaultColumnsRequest<'a> {
        SetIssueNavigatorDefaultColumnsRequest::new(self.client, column_request_body)
    }
}

/// Returns the default issue navigator columns.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
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
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/settings/columns".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<ColumnItem>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the default issue navigator columns.
///
/// The `columns` parameter accepts a navigable field value and is expressed as HTML form data. To specify multiple columns, pass multiple `columns` parameters. For example, in curl:
///
/// `curl -X PUT -d columns=summary -d columns=description https://your-domain.atlassian.net/rest/api/3/settings/columns`
///
/// If no column details are sent, then all default columns are removed.
///
/// A navigable field is one that can be used as a column on the issue navigator. Find details of navigable issue columns using [Get fields](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-issue-fields/#api-rest-api-3-field-get).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct SetIssueNavigatorDefaultColumnsRequest<'a> {
    client: &'a crate::core::Client,
    column_request_body: ColumnRequestBody,
}

impl<'a> SetIssueNavigatorDefaultColumnsRequest<'a> {
    fn new(client: &'a crate::core::Client, column_request_body: ColumnRequestBody) -> Self {
        Self { client, column_request_body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/settings/columns".to_owned());

        let body = match serde_json::to_value(&self.column_request_body)? {
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
