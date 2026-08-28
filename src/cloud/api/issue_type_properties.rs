// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueTypeProperties operations.
pub struct IssueTypePropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueTypePropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns all the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties) keys of the issue type.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the property keys of any issue type.
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the property keys of any issue types associated with the projects the user has permission to browse.
    pub fn get_issue_type_property_keys(
        &self,
        issue_type_id: impl Into<String>,
    ) -> GetIssueTypePropertyKeysRequest<'a> {
        GetIssueTypePropertyKeysRequest::new(self.client, issue_type_id)
    }

    /// Returns the key and value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the details of any issue type.
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the details of any issue types associated with the projects the user has permission to browse.
    pub fn get_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetIssueTypePropertyRequest<'a> {
        GetIssueTypePropertyRequest::new(self.client, issue_type_id, property_key)
    }

    /// Creates or updates the value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties). Use this resource to store and update data against an issue type.
    ///
    /// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn set_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetIssueTypePropertyRequest<'a> {
        SetIssueTypePropertyRequest::new(self.client, issue_type_id, property_key, body)
    }

    /// Deletes the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_issue_type_property(
        &self,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteIssueTypePropertyRequest<'a> {
        DeleteIssueTypePropertyRequest::new(self.client, issue_type_id, property_key)
    }
}

/// Returns all the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties) keys of the issue type.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the property keys of any issue type.
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the property keys of any issue types associated with the projects the user has permission to browse.
pub struct GetIssueTypePropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
}

impl<'a> GetIssueTypePropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/issuetype/{}/properties", crate::core::encode_path_segment(&self.issue_type_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the key and value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) to get the details of any issue type.
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) to get the details of any issue types associated with the projects the user has permission to browse.
pub struct GetIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
}

impl<'a> GetIssueTypePropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates or updates the value of the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties). Use this resource to store and update data against an issue type.
///
/// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct SetIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetIssueTypePropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        issue_type_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Deletes the [issue type property](https://developer.atlassian.com/cloud/jira/platform/storing-data-without-a-database/#a-id-jira-entity-properties-a-jira-entity-properties).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct DeleteIssueTypePropertyRequest<'a> {
    client: &'a crate::core::Client,
    issue_type_id: String,
    property_key: String,
}

impl<'a> DeleteIssueTypePropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, issue_type_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, issue_type_id: issue_type_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/issuetype/{}/properties/{}",
                crate::core::encode_path_segment(&self.issue_type_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
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
