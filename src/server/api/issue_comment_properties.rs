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

    /// Returns the keys of all properties for the comment identified by the key or by the id.
    pub fn get_comment_property_keys(&self, comment_id: impl Into<String>) -> GetCommentPropertyKeysRequest<'a> {
        GetCommentPropertyKeysRequest::new(self.client, comment_id)
    }

    /// Returns the value of the property with a given key from the comment identified by the key or by the id. The user who retrieves the property is required to have permissions to read the comment.
    pub fn get_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetCommentPropertyRequest<'a> {
        GetCommentPropertyRequest::new(self.client, comment_id, property_key)
    }

    /// Sets the value of the specified comment's property.
    pub fn set_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetCommentPropertyRequest<'a> {
        SetCommentPropertyRequest::new(self.client, comment_id, property_key, body)
    }

    /// Removes the property from the comment identified by the key or by the id. Ths user removing the property is required to have permissions to administer the comment.
    pub fn delete_comment_property(
        &self,
        comment_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteCommentPropertyRequest<'a> {
        DeleteCommentPropertyRequest::new(self.client, comment_id, property_key)
    }
}

/// Returns the keys of all properties for the comment identified by the key or by the id.
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
            format!("/rest/api/2/comment/{}/properties", crate::core::encode_path_segment(&self.comment_id)),
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

/// Returns the value of the property with a given key from the comment identified by the key or by the id. The user who retrieves the property is required to have permissions to read the comment.
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
                "/rest/api/2/comment/{}/properties/{}",
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

/// Sets the value of the specified comment's property.
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
                "/rest/api/2/comment/{}/properties/{}",
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

/// Removes the property from the comment identified by the key or by the id. Ths user removing the property is required to have permissions to administer the comment.
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
                "/rest/api/2/comment/{}/properties/{}",
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
