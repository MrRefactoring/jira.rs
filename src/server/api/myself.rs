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
