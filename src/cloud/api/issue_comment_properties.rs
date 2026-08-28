// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueCommentProperties operations.
pub struct IssueCommentPropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCommentPropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the keys of all the properties of a comment.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
    pub fn get_comment_property_keys(&self, comment_id: impl Into<String>) -> GetCommentPropertyKeysRequest<'a> {
        GetCommentPropertyKeysRequest::new(self.client, comment_id)
    }

    /// Returns the value of a comment property.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
    ///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
    ///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
    pub fn get_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetCommentPropertyRequest<'a> {
        GetCommentPropertyRequest::new(self.client, comment_id, property_key)
    }

    /// Creates or updates the value of a property for a comment. Use this resource to store custom data against a comment.
    ///
    /// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
    ///
    ///  *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on any comment.
    ///  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on a comment created by the user.
    pub fn set_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetCommentPropertyRequest<'a> {
        SetCommentPropertyRequest::new(self.client, comment_id, property_key, body)
    }

    /// Deletes a comment property.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
    ///
    ///  *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from any comment.
    ///  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from a comment created by the user.
    pub fn delete_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteCommentPropertyRequest<'a> {
        DeleteCommentPropertyRequest::new(self.client, comment_id, property_key)
    }
}

/// Returns the keys of all the properties of a comment.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
#[derive(Clone)]
pub struct GetCommentPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    comment_id: String,
}

impl<'a> GetCommentPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, comment_id: impl Into<String>) -> Self {
        Self { client, comment_id: comment_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/comment/{}/properties", crate::core::encode_path_segment(&self.comment_id)),
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

/// Returns the value of a comment property.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Browse projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for the project.
///  *  If [issue-level security](https://confluence.atlassian.com/x/J4lKLg) is configured, issue-level security permission to view the issue.
///  *  If the comment has visibility restrictions, belongs to the group or has the role visibility is restricted to.
#[derive(Clone)]
pub struct GetCommentPropertyRequest<'a> {
    client: &'a crate::core::Client,
    comment_id: String,
    property_key: String,
}

impl<'a> GetCommentPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, comment_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, comment_id: comment_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/comment/{}/properties/{}",
                crate::core::encode_path_segment(&self.comment_id),
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

/// Creates or updates the value of a property for a comment. Use this resource to store custom data against a comment.
///
/// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
///
///  *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on any comment.
///  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to create or update the value of a property on a comment created by the user.
#[derive(Clone)]
pub struct SetCommentPropertyRequest<'a> {
    client: &'a crate::core::Client,
    comment_id: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetCommentPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, comment_id: comment_id.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/comment/{}/properties/{}",
                crate::core::encode_path_segment(&self.comment_id),
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

/// Deletes a comment property.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** either of:
///
///  *  *Edit All Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from any comment.
///  *  *Edit Own Comments* [project permission](https://confluence.atlassian.com/x/yodKLg) to delete a property from a comment created by the user.
#[derive(Clone)]
pub struct DeleteCommentPropertyRequest<'a> {
    client: &'a crate::core::Client,
    comment_id: String,
    property_key: String,
}

impl<'a> DeleteCommentPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, comment_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, comment_id: comment_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/comment/{}/properties/{}",
                crate::core::encode_path_segment(&self.comment_id),
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
