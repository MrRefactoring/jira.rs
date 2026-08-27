// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    pub enum GetManagementPermissionsRequestPrivileges {
        Profile => "profile",
        ProfileWrite => "profile.write",
        ProfileRead => "profile.read",
        EmailSet => "email.set",
        LifecycleEnablement => "lifecycle.enablement",
        LifecycleDelete => "lifecycle.delete",
        ApiTokenRead => "apiToken.read",
        ApiTokenDelete => "apiToken.delete",
    }
}

/// The Manage operations.
pub struct ManageService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> ManageService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the set of permissions you have for managing the specified Atlassian account
    pub fn get_management_permissions(&self, account_id: AccountId) -> GetManagementPermissionsRequest<'a> {
        GetManagementPermissionsRequest::new(self.client, account_id)
    }
}

/// Returns the set of permissions you have for managing the specified Atlassian account
pub struct GetManagementPermissionsRequest<'a> {
    client: &'a crate::core::Client,
    account_id: AccountId,
    privileges: Option<Vec<GetManagementPermissionsRequestPrivileges>>,
}

impl<'a> GetManagementPermissionsRequest<'a> {
    fn new(client: &'a crate::core::Client, account_id: AccountId) -> Self {
        Self { client, account_id, privileges: None }
    }

    #[must_use]
    pub fn privileges(
        mut self,
        value: impl IntoIterator<Item = impl Into<GetManagementPermissionsRequestPrivileges>>,
    ) -> Self {
        self.privileges = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, format!("/users/{}/manage", self.account_id));

        if let Some(value) = &self.privileges {
            config.query.push(("privileges".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<GetManagementPermissions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
