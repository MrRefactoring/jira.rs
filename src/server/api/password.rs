// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Password operations.
pub struct PasswordService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> PasswordService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the list of requirements for the current password policy. For example, "The password must have at least 10 characters.", "The password must not be similar to the user's name or email address.", etc.
    pub fn get_password_policy(&self) -> GetPasswordPolicyRequest<'a> {
        GetPasswordPolicyRequest::new(self.client)
    }

    /// Returns a list of statements explaining why the password policy would disallow a proposed password for a new user.
    /// You can use this method to test the password policy validation. This could be done prior to an action
    /// where a new user and related password are created, using methods like the ones in
    /// [UserService](https://docs.atlassian.com/jira/latest/com/atlassian/jira/bc/user/UserService.html).
    /// For example, you could use this to validate a password in a create user form in the user interface, as the user enters it.
    /// The username and new password must be not empty to perform the validation.
    /// Note, this method will help you validate against the policy only. It won't check any other validations that might be performed
    /// when creating a new user, e.g. checking whether a user with the same name already exists.
    pub fn policy_check_create_user(
        &self,
        password_policy_create_user: PasswordPolicyCreateUser,
    ) -> PolicyCheckCreateUserRequest<'a> {
        PolicyCheckCreateUserRequest::new(self.client, password_policy_create_user)
    }

    /// Returns a list of statements explaining why the password policy would disallow a proposed new password for a user with an existing password.
    /// You can use this method to test the password policy validation. This could be done prior to an action where the password
    /// is actually updated, using methods like ChangePassword or ResetPassword.
    /// For example, you could use this to validate a password in a change password form in the user interface, as the user enters it.
    /// The user must exist and the username and new password must be not empty, to perform the validation.
    /// Note, this method will help you validate against the policy only. It won't check any other validations that might be performed
    /// when submitting a password change/reset request, e.g. verifying whether the old password is valid.
    pub fn policy_check_update_user(
        &self,
        password_policy_update_user: PasswordPolicyUpdateUser,
    ) -> PolicyCheckUpdateUserRequest<'a> {
        PolicyCheckUpdateUserRequest::new(self.client, password_policy_update_user)
    }
}

/// Returns the list of requirements for the current password policy. For example, "The password must have at least 10 characters.", "The password must not be similar to the user's name or email address.", etc.
pub struct GetPasswordPolicyRequest<'a> {
    client: &'a crate::core::Client,
    has_old_password: Option<bool>,
}

impl<'a> GetPasswordPolicyRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, has_old_password: None }
    }

    /// Whether or not the user will be required to enter their current password.  Use false (the default) if this is a new user or if an administrator is forcibly changing another user's password.
    #[must_use]
    pub fn has_old_password(mut self, value: bool) -> Self {
        self.has_old_password = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/password/policy".to_owned());

        if let Some(value) = &self.has_old_password {
            config.query.push(("hasOldPassword".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of statements explaining why the password policy would disallow a proposed password for a new user.
/// You can use this method to test the password policy validation. This could be done prior to an action
/// where a new user and related password are created, using methods like the ones in
/// [UserService](https://docs.atlassian.com/jira/latest/com/atlassian/jira/bc/user/UserService.html).
/// For example, you could use this to validate a password in a create user form in the user interface, as the user enters it.
/// The username and new password must be not empty to perform the validation.
/// Note, this method will help you validate against the policy only. It won't check any other validations that might be performed
/// when creating a new user, e.g. checking whether a user with the same name already exists.
pub struct PolicyCheckCreateUserRequest<'a> {
    client: &'a crate::core::Client,
    password_policy_create_user: PasswordPolicyCreateUser,
}

impl<'a> PolicyCheckCreateUserRequest<'a> {
    fn new(client: &'a crate::core::Client, password_policy_create_user: PasswordPolicyCreateUser) -> Self {
        Self { client, password_policy_create_user }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/password/policy/createUser".to_owned(),
        );

        let body = match serde_json::to_value(&self.password_policy_create_user)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of statements explaining why the password policy would disallow a proposed new password for a user with an existing password.
/// You can use this method to test the password policy validation. This could be done prior to an action where the password
/// is actually updated, using methods like ChangePassword or ResetPassword.
/// For example, you could use this to validate a password in a change password form in the user interface, as the user enters it.
/// The user must exist and the username and new password must be not empty, to perform the validation.
/// Note, this method will help you validate against the policy only. It won't check any other validations that might be performed
/// when submitting a password change/reset request, e.g. verifying whether the old password is valid.
pub struct PolicyCheckUpdateUserRequest<'a> {
    client: &'a crate::core::Client,
    password_policy_update_user: PasswordPolicyUpdateUser,
}

impl<'a> PolicyCheckUpdateUserRequest<'a> {
    fn new(client: &'a crate::core::Client, password_policy_update_user: PasswordPolicyUpdateUser) -> Self {
        Self { client, password_policy_update_user }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/2/password/policy/updateUser".to_owned(),
        );

        let body = match serde_json::to_value(&self.password_policy_update_user)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<serde_json::Value> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
