// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Myself operations.
pub struct MyselfService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> MyselfService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns preference of the currently logged in user. Preference key must be provided as input parameter (key). The value is returned exactly as it is. If key parameter is not provided or wrong - status code 404. If value is found  - status code 200.
    pub fn get_preference(&self, key: impl Into<String>) -> GetPreferenceRequest<'a> {
        GetPreferenceRequest::new(self.client, key)
    }

    /// Sets preference of the currently logged in user. Preference key must be provided as input parameters (key). Value must be provided as post body. If key or value parameter is not provided - status code 404. If preference is set - status code 204.
    pub fn set_preference(&self, key: impl Into<String>, body: impl Into<String>) -> SetPreferenceRequest<'a> {
        SetPreferenceRequest::new(self.client, key, body)
    }

    /// Removes preference of the currently logged in user. Preference key must be provided as input parameters (key). If key parameter is not provided or wrong - status code 404. If preference is unset - status code 204.
    pub fn remove_preference(&self, key: impl Into<String>) -> RemovePreferenceRequest<'a> {
        RemovePreferenceRequest::new(self.client, key)
    }

    /// Returns currently logged user. This resource cannot be accessed anonymously
    pub fn get_current_user(&self) -> GetCurrentUserRequest<'a> {
        GetCurrentUserRequest::new(self.client)
    }

    /// Modify currently logged user. The 'value' fields present will override the existing value. Fields skipped in request will not be changed. Only email and display name can be change that way. Requires user password.
    pub fn update_current_user(&self, user_write: UserWrite) -> UpdateCurrentUserRequest<'a> {
        UpdateCurrentUserRequest::new(self.client, user_write)
    }

    /// Modify caller password.
    pub fn change_my_password(&self, password: Password) -> ChangeMyPasswordRequest<'a> {
        ChangeMyPasswordRequest::new(self.client, password)
    }
}

/// Returns preference of the currently logged in user. Preference key must be provided as input parameter (key). The value is returned exactly as it is. If key parameter is not provided or wrong - status code 404. If value is found  - status code 200.
#[derive(Clone)]
pub struct GetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
}

impl<'a> GetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>) -> Self {
        Self { client, key: key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<String> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets preference of the currently logged in user. Preference key must be provided as input parameters (key). Value must be provided as post body. If key or value parameter is not provided - status code 404. If preference is set - status code 204.
#[derive(Clone)]
pub struct SetPreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
    body: String,
}

impl<'a> SetPreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>, body: impl Into<String>) -> Self {
        Self { client, key: key.into(), body: body.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

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

/// Removes preference of the currently logged in user. Preference key must be provided as input parameters (key). If key parameter is not provided or wrong - status code 404. If preference is unset - status code 204.
#[derive(Clone)]
pub struct RemovePreferenceRequest<'a> {
    client: &'a crate::core::Client,
    key: String,
}

impl<'a> RemovePreferenceRequest<'a> {
    fn new(client: &'a crate::core::Client, key: impl Into<String>) -> Self {
        Self { client, key: key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/mypreferences".to_owned());

        config.query.push(("key".to_owned(), crate::core::QueryValue::Scalar(self.key.clone())));

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

/// Returns currently logged user. This resource cannot be accessed anonymously
#[derive(Clone)]
pub struct GetCurrentUserRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetCurrentUserRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/myself".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<User> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Modify currently logged user. The 'value' fields present will override the existing value. Fields skipped in request will not be changed. Only email and display name can be change that way. Requires user password.
#[derive(Clone)]
pub struct UpdateCurrentUserRequest<'a> {
    client: &'a crate::core::Client,
    user_write: UserWrite,
}

impl<'a> UpdateCurrentUserRequest<'a> {
    fn new(client: &'a crate::core::Client, user_write: UserWrite) -> Self {
        Self { client, user_write }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/myself".to_owned());

        let body = match serde_json::to_value(&self.user_write)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UserWrite> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Modify caller password.
#[derive(Clone)]
pub struct ChangeMyPasswordRequest<'a> {
    client: &'a crate::core::Client,
    password: Password,
}

impl<'a> ChangeMyPasswordRequest<'a> {
    fn new(client: &'a crate::core::Client, password: Password) -> Self {
        Self { client, password }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/2/myself/password".to_owned());

        let body = match serde_json::to_value(&self.password)? {
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
