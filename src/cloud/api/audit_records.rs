// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The AuditRecords operations.
pub struct AuditRecordsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AuditRecordsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of audit records. The list can be filtered to include items:
    ///
    ///  *  where each item in `filter` has at least one match in any of these fields:
    ///
    ///      *  `summary`
    ///      *  `category`
    ///      *  `eventSource`
    ///      *  `objectItem.name` If the object is a user, account ID is available to filter.
    ///      *  `objectItem.parentName`
    ///      *  `objectItem.typeName`
    ///      *  `changedValues.changedFrom`
    ///      *  `changedValues.changedTo`
    ///      *  `remoteAddress`
    ///
    ///     For example, if `filter` contains *man ed*, an audit record containing `summary": "User added to group"` and `"category": "group management"` is returned.
    ///  *  created on or after a date and time.
    ///  *  created or or before a date and time.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_audit_records(&self) -> GetAuditRecordsRequest<'a> {
        GetAuditRecordsRequest::new(self.client)
    }
}

/// Returns a list of audit records. The list can be filtered to include items:
///
///  *  where each item in `filter` has at least one match in any of these fields:
///
///      *  `summary`
///      *  `category`
///      *  `eventSource`
///      *  `objectItem.name` If the object is a user, account ID is available to filter.
///      *  `objectItem.parentName`
///      *  `objectItem.typeName`
///      *  `changedValues.changedFrom`
///      *  `changedValues.changedTo`
///      *  `remoteAddress`
///
///     For example, if `filter` contains *man ed*, an audit record containing `summary": "User added to group"` and `"category": "group management"` is returned.
///  *  created on or after a date and time.
///  *  created or or before a date and time.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetAuditRecordsRequest<'a> {
    client: &'a crate::core::Client,
    offset: Option<i64>,
    limit: Option<i64>,
    filter: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

impl<'a> GetAuditRecordsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, offset: None, limit: None, filter: None, from: None, to: None }
    }

    /// The number of records to skip before returning the first result.
    #[must_use]
    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);

        self
    }

    /// The maximum number of results to return.
    #[must_use]
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);

        self
    }

    /// The strings to match with audit field content, space separated.
    #[must_use]
    pub fn filter(mut self, value: impl Into<String>) -> Self {
        self.filter = Some(value.into());

        self
    }

    /// The date and time on or after which returned audit records must have been created. If `to` is provided `from` must be before `to` or no audit records are returned.
    #[must_use]
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());

        self
    }

    /// The date and time on or before which returned audit results must have been created. If `from` is provided `to` must be after `from` or no audit records are returned.
    #[must_use]
    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/auditing/record".to_owned());

        if let Some(value) = &self.offset {
            config.query.push(("offset".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.limit {
            config.query.push(("limit".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.filter {
            config.query.push(("filter".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.from {
            config.query.push(("from".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.to {
            config.query.push(("to".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AuditRecords> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
