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

    /// Returns the keys of all properties for a user.
    ///
    /// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to access the property keys on any user.
    ///  *  Access to Jira, to access the calling user's property keys.
    pub fn get_user_property_keys(&self) -> GetUserPropertyKeysRequest<'a> {
        GetUserPropertyKeysRequest::new(self.client)
    }

    /// Returns the value of a user's property. If no property key is provided [Get user property keys](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-user/#api-rest-api-3-user-properties-get) is called.
    ///
    /// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get a property from any user.
    ///  *  Access to Jira, to get a property from the calling user's record.
    pub fn get_user_property(&self, property_key: impl Into<String>) -> GetUserPropertyRequest<'a> {
        GetUserPropertyRequest::new(self.client, property_key)
    }

    /// Sets the value of a user's property. Use this resource to store custom data against a user.
    ///
    /// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to set a property on any user.
    ///  *  Access to Jira, to set a property on the calling user's record.
    pub fn set_user_property(
        &self,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetUserPropertyRequest<'a> {
        SetUserPropertyRequest::new(self.client, property_key, body)
    }

    /// Deletes a property from a user.
    ///
    /// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
    ///
    ///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to delete a property from any user.
    ///  *  Access to Jira, to delete a property from the calling user's record.
    pub fn delete_user_property(&self, property_key: impl Into<String>) -> DeleteUserPropertyRequest<'a> {
        DeleteUserPropertyRequest::new(self.client, property_key)
    }
}

/// Returns the keys of all properties for a user.
///
/// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to access the property keys on any user.
///  *  Access to Jira, to access the calling user's property keys.
pub struct GetUserPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    account_id: Option<String>,
}

impl<'a> GetUserPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, account_id: None }
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/user/properties".to_owned());

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

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

/// Returns the value of a user's property. If no property key is provided [Get user property keys](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-user/#api-rest-api-3-user-properties-get) is called.
///
/// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get a property from any user.
///  *  Access to Jira, to get a property from the calling user's record.
pub struct GetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: Option<String>,
    property_key: String,
}

impl<'a> GetUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), account_id: None }
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Sets the value of a user's property. Use this resource to store custom data against a user.
///
/// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to set a property on any user.
///  *  Access to Jira, to set a property on the calling user's record.
pub struct SetUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: Option<String>,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetUserPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), body, account_id: None }
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/api/3/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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

/// Deletes a property from a user.
///
/// Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:**
///
///  *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to delete a property from any user.
///  *  Access to Jira, to delete a property from the calling user's record.
pub struct DeleteUserPropertyRequest<'a> {
    client: &'a crate::core::Client,
    account_id: Option<String>,
    property_key: String,
}

impl<'a> DeleteUserPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into(), account_id: None }
    }

    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*.
    #[must_use]
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/api/3/user/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        if let Some(value) = &self.account_id {
            config.query.push(("accountId".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
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
