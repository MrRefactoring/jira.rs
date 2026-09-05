// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The AppProperties operations.
pub struct AppPropertiesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AppPropertiesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Gets all the properties of an app. The reserved key `connect_client_key_019cdff3-8bfb-71fe-9628-875b700aebb8` is not returned.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
    /// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
    pub fn get_addon_properties(&self, addon_key: impl Into<String>) -> GetAddonPropertiesRequest<'a> {
        GetAddonPropertiesRequest::new(self.client, addon_key)
    }

    /// Returns the key and value of an app's property. The property key `connect_client_key_019cdff3-8bfb-71fe-9628-875b700aebb8`
    /// is reserved. It returns a synthetic, read-only property containing the Connect `clientKey` for the requested tenant.
    /// This is intended for Forge apps with `app.connect.key` to retrieve the Connect client key during migration.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
    /// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
    pub fn get_addon_property(
        &self,
        addon_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetAddonPropertyRequest<'a> {
        GetAddonPropertyRequest::new(self.client, addon_key, property_key)
    }

    /// Sets the value of an app's property. Use this resource to store custom data for your app.
    ///
    /// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
    /// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
    pub fn put_addon_property(
        &self,
        addon_key: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> PutAddonPropertyRequest<'a> {
        PutAddonPropertyRequest::new(self.client, addon_key, property_key, body)
    }

    /// Deletes an app's property.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
    /// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
    pub fn delete_addon_property(
        &self,
        addon_key: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeleteAddonPropertyRequest<'a> {
        DeleteAddonPropertyRequest::new(self.client, addon_key, property_key)
    }

    /// Returns all property keys for the Forge app.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
    pub fn get_forge_app_property_keys(&self) -> GetForgeAppPropertyKeysRequest<'a> {
        GetForgeAppPropertyKeysRequest::new(self.client)
    }

    /// Returns the value of a Forge app's property.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
    pub fn get_forge_app_property(&self, property_key: impl Into<String>) -> GetForgeAppPropertyRequest<'a> {
        GetForgeAppPropertyRequest::new(self.client, property_key)
    }

    /// Sets the value of a Forge app's property.
    /// These values can be retrieved in [Jira expressions](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/)
    /// through the `app` [context variable](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#context-variables).
    /// They are also available in [entity property display conditions](https://developer.atlassian.com/platform/forge/manifest-reference/display-conditions/entity-property-conditions/).
    ///
    /// For other use cases, use the [Storage API](https://developer.atlassian.com/platform/forge/runtime-reference/storage-api/).
    ///
    /// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
    ///
    /// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
    pub fn put_forge_app_property(
        &self,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> PutForgeAppPropertyRequest<'a> {
        PutForgeAppPropertyRequest::new(self.client, property_key, body)
    }

    /// Deletes a Forge app's property.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
    ///
    /// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
    pub fn delete_forge_app_property(&self, property_key: impl Into<String>) -> DeleteForgeAppPropertyRequest<'a> {
        DeleteForgeAppPropertyRequest::new(self.client, property_key)
    }
}

/// Gets all the properties of an app. The reserved key `connect_client_key_019cdff3-8bfb-71fe-9628-875b700aebb8` is not returned.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
/// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
#[derive(Clone)]
pub struct GetAddonPropertiesRequest<'a> {
    client: &'a crate::core::Client,
    addon_key: String,
}

impl<'a> GetAddonPropertiesRequest<'a> {
    fn new(client: &'a crate::core::Client, addon_key: impl Into<String>) -> Self {
        Self { client, addon_key: addon_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/atlassian-connect/1/addons/{}/properties",
                crate::core::encode_path_segment(&self.addon_key)
            ),
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

/// Returns the key and value of an app's property. The property key `connect_client_key_019cdff3-8bfb-71fe-9628-875b700aebb8`
/// is reserved. It returns a synthetic, read-only property containing the Connect `clientKey` for the requested tenant.
/// This is intended for Forge apps with `app.connect.key` to retrieve the Connect client key during migration.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
/// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
#[derive(Clone)]
pub struct GetAddonPropertyRequest<'a> {
    client: &'a crate::core::Client,
    addon_key: String,
    property_key: String,
}

impl<'a> GetAddonPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, addon_key: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, addon_key: addon_key.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/atlassian-connect/1/addons/{}/properties/{}",
                crate::core::encode_path_segment(&self.addon_key),
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

/// Sets the value of an app's property. Use this resource to store custom data for your app.
///
/// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
/// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
#[derive(Clone)]
pub struct PutAddonPropertyRequest<'a> {
    client: &'a crate::core::Client,
    addon_key: String,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> PutAddonPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        addon_key: impl Into<String>,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, addon_key: addon_key.into(), property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/atlassian-connect/1/addons/{}/properties/{}",
                crate::core::encode_path_segment(&self.addon_key),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<OperationMessage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an app's property.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.
/// Additionally, Forge apps can access Connect app properties (stored against the same `app.connect.key`).
#[derive(Clone)]
pub struct DeleteAddonPropertyRequest<'a> {
    client: &'a crate::core::Client,
    addon_key: String,
    property_key: String,
}

impl<'a> DeleteAddonPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, addon_key: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, addon_key: addon_key.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/atlassian-connect/1/addons/{}/properties/{}",
                crate::core::encode_path_segment(&self.addon_key),
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

/// Returns all property keys for the Forge app.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
#[derive(Clone)]
pub struct GetForgeAppPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetForgeAppPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/forge/1/app/properties".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetForgeAppPropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of a Forge app's property.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
#[derive(Clone)]
pub struct GetForgeAppPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
}

impl<'a> GetForgeAppPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/forge/1/app/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetForgeAppProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of a Forge app's property.
/// These values can be retrieved in [Jira expressions](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/)
/// through the `app` [context variable](https://developer.atlassian.com/cloud/jira/platform/jira-expressions/#context-variables).
/// They are also available in [entity property display conditions](https://developer.atlassian.com/platform/forge/manifest-reference/display-conditions/entity-property-conditions/).
///
/// For other use cases, use the [Storage API](https://developer.atlassian.com/platform/forge/runtime-reference/storage-api/).
///
/// The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
///
/// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
#[derive(Clone)]
pub struct PutForgeAppPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> PutForgeAppPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/forge/1/app/properties/{}", crate::core::encode_path_segment(&self.property_key)),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<OperationMessage> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a Forge app's property.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Forge apps can make this request. This API can only be accessed using **[asApp()](https://developer.atlassian.com/platform/forge/apis-reference/fetch-api-product.requestjira/#method-signature)** requests from Forge.
///
/// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
#[derive(Clone)]
pub struct DeleteForgeAppPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
}

impl<'a> DeleteForgeAppPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>) -> Self {
        Self { client, property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/forge/1/app/properties/{}", crate::core::encode_path_segment(&self.property_key)),
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
