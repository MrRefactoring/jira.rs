// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Email operations.
pub struct EmailService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> EmailService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Sets the specified user's email address. Before using this endpoint, you must [verify the target domain](https://confluence.atlassian.com/x/gjcWN) as the new email address will be considered verified.
    /// The permission to make use of this resource is exposed by the `email.set` privilege.
    /// This call invalidates all active sessions.
    pub fn set_email(&self, account_id: AccountId, email: Email) -> SetEmailRequest<'a> {
        SetEmailRequest::new(self.client, account_id, email)
    }
}

/// Sets the specified user's email address. Before using this endpoint, you must [verify the target domain](https://confluence.atlassian.com/x/gjcWN) as the new email address will be considered verified.
/// The permission to make use of this resource is exposed by the `email.set` privilege.
/// This call invalidates all active sessions.
#[derive(Clone)]
pub struct SetEmailRequest<'a> {
    client: &'a crate::core::Client,
    account_id: AccountId,
    email: Email,
}

impl<'a> SetEmailRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: AccountId, email: Email) -> Self {
        Self { client, account_id, email }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/users/{}/manage/email", crate::core::encode_path_segment(&self.account_id)),
        );

        let mut body = serde_json::Map::new();

        body.insert("email".to_owned(), serde_json::to_value(&self.email)?);

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
