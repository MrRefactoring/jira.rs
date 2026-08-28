// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueLinkTypes operations.
pub struct IssueLinkTypesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueLinkTypesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of all issue link types.
    ///
    /// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.
    pub fn get_issue_link_types(&self) -> GetIssueLinkTypesRequest<'a> {
        GetIssueLinkTypesRequest::new(self.client)
    }

    /// Creates an issue link type. Use this operation to create descriptions of the reasons why issues are linked. The issue link type consists of a name and descriptions for a link's inward and outward relationships.
    ///
    /// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_issue_link_type(&self, issue_link_type: IssueLinkType) -> CreateIssueLinkTypeRequest<'a> {
        CreateIssueLinkTypeRequest::new(self.client, issue_link_type)
    }

    /// Returns an issue link type.
    ///
    /// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.
    pub fn get_issue_link_type(&self, issue_link_type_id: impl Into<String>) -> GetIssueLinkTypeRequest<'a> {
        GetIssueLinkTypeRequest::new(self.client, issue_link_type_id)
    }

    /// Updates an issue link type.
    ///
    /// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn update_issue_link_type(
        &self,
        issue_link_type_id: impl Into<String>,
        issue_link_type: IssueLinkType,
    ) -> UpdateIssueLinkTypeRequest<'a> {
        UpdateIssueLinkTypeRequest::new(self.client, issue_link_type_id, issue_link_type)
    }

    /// Deletes an issue link type.
    ///
    /// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_issue_link_type(&self, issue_link_type_id: impl Into<String>) -> DeleteIssueLinkTypeRequest<'a> {
        DeleteIssueLinkTypeRequest::new(self.client, issue_link_type_id)
    }
}

/// Returns a list of all issue link types.
///
/// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.
pub struct GetIssueLinkTypesRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetIssueLinkTypesRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/issueLinkType".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkTypes> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates an issue link type. Use this operation to create descriptions of the reasons why issues are linked. The issue link type consists of a name and descriptions for a link's inward and outward relationships.
///
/// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct CreateIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type: IssueLinkType,
}

impl<'a> CreateIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type: IssueLinkType) -> Self {
        Self { client, issue_link_type }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/issueLinkType".to_owned());

        let body = match serde_json::to_value(&self.issue_link_type)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns an issue link type.
///
/// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project in the site.
pub struct GetIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
}

impl<'a> GetIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type_id: impl Into<String>) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates an issue link type.
///
/// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct UpdateIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
    issue_link_type: IssueLinkType,
}

impl<'a> UpdateIssueLinkTypeRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_link_type_id: impl Into<String>,
        issue_link_type: IssueLinkType,
    ) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into(), issue_link_type }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
        );

        let body = match serde_json::to_value(&self.issue_link_type)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueLinkType> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an issue link type.
///
/// To use this operation, the site must have [issue linking](https://confluence.atlassian.com/x/yoXKM) enabled.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct DeleteIssueLinkTypeRequest<'a> {
    client: &'a crate::core::Client,
    issue_link_type_id: String,
}

impl<'a> DeleteIssueLinkTypeRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_link_type_id: impl Into<String>) -> Self {
        Self { client, issue_link_type_id: issue_link_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/issueLinkType/{}", crate::core::encode_path_segment(&self.issue_link_type_id)),
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
