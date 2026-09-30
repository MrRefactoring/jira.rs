// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Profile operations.
pub struct ProfileService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ProfileService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns information about a single Atlassian account by ID
    pub fn get_profile(&self, account_id: AccountId) -> GetProfileRequest<'a> {
        GetProfileRequest::new(self.client, account_id)
    }

    /// Updates fields in a user account. The `profile.write` privilege details which fields you can change.
    pub fn update_profile(&self, account_id: AccountId) -> UpdateProfileRequest<'a> {
        UpdateProfileRequest::new(self.client, account_id)
    }
}

/// Returns information about a single Atlassian account by ID
#[derive(Clone)]
pub struct GetProfileRequest<'a> {
    client: &'a crate::core::Client,
    account_id: AccountId,
}

impl<'a> GetProfileRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: AccountId) -> Self {
        Self { client, account_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/users/{}/manage/profile", crate::core::encode_path_segment(&self.account_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetProfile> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates fields in a user account. The `profile.write` privilege details which fields you can change.
#[derive(Clone)]
pub struct UpdateProfileRequest<'a> {
    client: &'a crate::core::Client,
    account_id: AccountId,
    name: Option<Name>,
    nickname: Option<Nickname>,
    zoneinfo: Option<ZoneInfo>,
    locale: Option<Locale>,
    extended_profile: Option<ExtendedProfile>,
}

impl<'a> UpdateProfileRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: AccountId) -> Self {
        Self { client, account_id, name: None, nickname: None, zoneinfo: None, locale: None, extended_profile: None }
    }

    #[must_use]
    pub fn name(mut self, value: Name) -> Self {
        self.name = Some(value);

        self
    }

    #[must_use]
    pub fn nickname(mut self, value: Nickname) -> Self {
        self.nickname = Some(value);

        self
    }

    #[must_use]
    pub fn zoneinfo(mut self, value: ZoneInfo) -> Self {
        self.zoneinfo = Some(value);

        self
    }

    #[must_use]
    pub fn locale(mut self, value: Locale) -> Self {
        self.locale = Some(value);

        self
    }

    #[must_use]
    pub fn extended_profile(mut self, value: ExtendedProfile) -> Self {
        self.extended_profile = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PATCH,
            format!("/users/{}/manage/profile", crate::core::encode_path_segment(&self.account_id)),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.name {
            body.insert("name".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.nickname {
            body.insert("nickname".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.zoneinfo {
            body.insert("zoneinfo".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.locale {
            body.insert("locale".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.extended_profile {
            body.insert("extended_profile".to_owned(), serde_json::to_value(value)?);
        }

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<UpdateProfile> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
