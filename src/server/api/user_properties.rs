// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The UserProperties operations.
pub struct UserPropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UserPropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the keys of all properties for the user identified by the key or by the id.
    pub fn get_user_property_keys(&self) -> GetUserPropertyKeysRequest<'a> {
        GetUserPropertyKeysRequest::new(self.client)
    }

    /// Returns the value of the property with a given key from the user identified by the key or by the id.
    pub fn get_user_property(&self, property_key: impl Into<String>) -> GetUserPropertyRequest<'a> {
        GetUserPropertyRequest::new(self.client, property_key)
    }

    /// Sets the value of the specified user's property.
    /// You can use this resource to store a custom data against the user identified by the key or by the id. The user
    /// who stores the data is required to have permissions to administer the user.
    pub fn set_user_property(
        &self,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetUserPropertyRequest<'a> {
        SetUserPropertyRequest::new(self.client, property_key, body)
    }

    /// Removes the property from the user identified by the key or by the id. The user who removes the property is required to have permissions to administer the user.
    pub fn delete_user_property(&self, property_key: impl Into<String>) -> DeleteUserPropertyRequest<'a> {
        DeleteUserPropertyRequest::new(self.client, property_key)
    }
}

/// Returns the keys of all properties for the user identified by the key or by the id.
#[derive(Clone)]
pub struct GetUserPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> GetUserPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, user_key: None, username: None }
    }

    /// Key of the user whose properties are to be returned
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose properties are to be returned
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/user/properties".to_owned());

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityPropertiesKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the user identified by the key or by the id.
#[derive(Clone)]
pub struct GetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> GetUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), user_key: None, username: None }
    }

    /// Key of the user whose property is to be returned
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be returned
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Sets the value of the specified user's property.
/// You can use this resource to store a custom data against the user identified by the key or by the id. The user
/// who stores the data is required to have permissions to administer the user.
#[derive(Clone)]
pub struct SetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetUserPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), body, user_key: None, username: None }
    }

    /// Key of the user whose property is to be set
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be set
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Removes the property from the user identified by the key or by the id. The user who removes the property is required to have permissions to administer the user.
#[derive(Clone)]
pub struct DeleteUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    user_key: Option<String>,
    username: Option<String>,
}

impl<'a> DeleteUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), user_key: None, username: None }
    }

    /// Key of the user whose property is to be removed
    #[must_use]
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());

        self
    }

    /// Username of the user whose property is to be removed
    #[must_use]
    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/2/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.user_key {
            config.query.push(("userKey".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.username {
            config.query.push(("username".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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
