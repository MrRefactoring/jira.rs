// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueCustomFieldOptionsApps operations.
pub struct IssueCustomFieldOptionsAppsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCustomFieldOptionsAppsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all the options of a select list issue field. A select list issue field is a type of [issue field](https://developer.atlassian.com/cloud/jira/platform/modules/issue-field/) that enables a user to select a value from a list of options.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn get_all_issue_field_options(&self, field_key: impl Into<String>) -> GetAllIssueFieldOptionsRequest<'a> {
        GetAllIssueFieldOptionsRequest::new(self.client, field_key)
    }

    /// Creates an option for a select list issue field.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// Each field can have a maximum of 10000 options, and each option can have a maximum of 10000 scopes.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn create_issue_field_option(
        &self,
        field_key: impl Into<String>,
        issue_field_option_create: IssueFieldOptionCreate,
    ) -> CreateIssueFieldOptionRequest<'a> {
        CreateIssueFieldOptionRequest::new(self.client, field_key, issue_field_option_create)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of options for a select list issue field that can be viewed and selected by the user.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_selectable_issue_field_options(
        &self,
        field_key: impl Into<String>,
    ) -> GetSelectableIssueFieldOptionsRequest<'a> {
        GetSelectableIssueFieldOptionsRequest::new(self.client, field_key)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of options for a select list issue field that can be viewed by the user.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
    pub fn get_visible_issue_field_options(
        &self,
        field_key: impl Into<String>,
    ) -> GetVisibleIssueFieldOptionsRequest<'a> {
        GetVisibleIssueFieldOptionsRequest::new(self.client, field_key)
    }

    /// Returns an option from a select list issue field.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn get_issue_field_option(
        &self,
        field_key: impl Into<String>,
        option_id: i64,
    ) -> GetIssueFieldOptionRequest<'a> {
        GetIssueFieldOptionRequest::new(self.client, field_key, option_id)
    }

    /// Updates or creates an option for a select list issue field. This operation requires that the option ID is provided when creating an option, therefore, the option ID needs to be specified as a path and body parameter. The option ID provided in the path and body must be identical.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn update_issue_field_option(
        &self,
        field_key: impl Into<String>,
        option_id: i64,
        issue_field_option: IssueFieldOption,
    ) -> UpdateIssueFieldOptionRequest<'a> {
        UpdateIssueFieldOptionRequest::new(self.client, field_key, option_id, issue_field_option)
    }

    /// Deletes an option from a select list issue field.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn delete_issue_field_option(
        &self,
        field_key: impl Into<String>,
        option_id: i64,
    ) -> DeleteIssueFieldOptionRequest<'a> {
        DeleteIssueFieldOptionRequest::new(self.client, field_key, option_id)
    }

    /// Deselects an issue-field select-list option from all issues where it is selected. A different option can be selected to replace the deselected option. The update can also be limited to a smaller set of issues by using a JQL query.
    ///
    /// Connect and Forge app users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) can override the screen security configuration using `overrideScreenSecurity` and `overrideEditableFlag`.
    ///
    /// This is an [asynchronous operation](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#async). The response object contains a link to the long-running task.
    ///
    /// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
    pub fn replace_issue_field_option(
        &self,
        field_key: impl Into<String>,
        option_id: i64,
    ) -> ReplaceIssueFieldOptionRequest<'a> {
        ReplaceIssueFieldOptionRequest::new(self.client, field_key, option_id)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of all the options of a select list issue field. A select list issue field is a type of [issue field](https://developer.atlassian.com/cloud/jira/platform/modules/issue-field/) that enables a user to select a value from a list of options.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct GetAllIssueFieldOptionsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    field_key: String,
}

impl<'a> GetAllIssueFieldOptionsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>) -> Self {
        Self { client, field_key: field_key.into(), start_at: None, max_results: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/field/{}/option", crate::core::encode_path_segment(&self.field_key)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<IssueFieldOption>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates an option for a select list issue field.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// Each field can have a maximum of 10000 options, and each option can have a maximum of 10000 scopes.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct CreateIssueFieldOptionRequest<'a> {
    client: &'a crate::core::Client,
    field_key: String,
    issue_field_option_create: IssueFieldOptionCreate,
}

impl<'a> CreateIssueFieldOptionRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_key: impl Into<String>,
        issue_field_option_create: IssueFieldOptionCreate,
    ) -> Self {
        Self { client, field_key: field_key.into(), issue_field_option_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/field/{}/option", crate::core::encode_path_segment(&self.field_key)),
        );

        let body = match serde_json::to_value(&self.issue_field_option_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueFieldOption> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of options for a select list issue field that can be viewed and selected by the user.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetSelectableIssueFieldOptionsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    project_id: Option<i64>,
    field_key: String,
}

impl<'a> GetSelectableIssueFieldOptionsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>) -> Self {
        Self { client, field_key: field_key.into(), start_at: None, max_results: None, project_id: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters the results to options that are only available in the specified project.
    #[must_use]
    pub fn project_id(mut self, value: i64) -> Self {
        self.project_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/3/field/{}/option/suggestions/edit", crate::core::encode_path_segment(&self.field_key)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<IssueFieldOption>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of options for a select list issue field that can be viewed by the user.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Permission to access Jira.
#[derive(Clone)]
pub struct GetVisibleIssueFieldOptionsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    project_id: Option<i64>,
    field_key: String,
}

impl<'a> GetVisibleIssueFieldOptionsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>) -> Self {
        Self { client, field_key: field_key.into(), start_at: None, max_results: None, project_id: None }
    }

    /// The index of the first item to return in a page of results (page offset).
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The maximum number of items to return per page.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Filters the results to options that are only available in the specified project.
    #[must_use]
    pub fn project_id(mut self, value: i64) -> Self {
        self.project_id = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/field/{}/option/suggestions/search",
                crate::core::encode_path_segment(&self.field_key)
            ),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.project_id {
            config.query.push(("projectId".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<IssueFieldOption>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns an option from a select list issue field.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct GetIssueFieldOptionRequest<'a> {
    client: &'a crate::core::Client,
    field_key: String,
    option_id: i64,
}

impl<'a> GetIssueFieldOptionRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>, option_id: i64) -> Self {
        Self { client, field_key: field_key.into(), option_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/api/3/field/{}/option/{}",
                crate::core::encode_path_segment(&self.field_key),
                self.option_id
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueFieldOption> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates or creates an option for a select list issue field. This operation requires that the option ID is provided when creating an option, therefore, the option ID needs to be specified as a path and body parameter. The option ID provided in the path and body must be identical.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct UpdateIssueFieldOptionRequest<'a> {
    client: &'a crate::core::Client,
    field_key: String,
    option_id: i64,
    issue_field_option: IssueFieldOption,
}

impl<'a> UpdateIssueFieldOptionRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_key: impl Into<String>,
        option_id: i64,
        issue_field_option: IssueFieldOption,
    ) -> Self {
        Self { client, field_key: field_key.into(), option_id, issue_field_option }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/field/{}/option/{}",
                crate::core::encode_path_segment(&self.field_key),
                self.option_id
            ),
        );

        let body = match serde_json::to_value(&self.issue_field_option)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<IssueFieldOption> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes an option from a select list issue field.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct DeleteIssueFieldOptionRequest<'a> {
    client: &'a crate::core::Client,
    field_key: String,
    option_id: i64,
}

impl<'a> DeleteIssueFieldOptionRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>, option_id: i64) -> Self {
        Self { client, field_key: field_key.into(), option_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/field/{}/option/{}",
                crate::core::encode_path_segment(&self.field_key),
                self.option_id
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

/// Deselects an issue-field select-list option from all issues where it is selected. A different option can be selected to replace the deselected option. The update can also be limited to a smaller set of issues by using a JQL query.
///
/// Connect and Forge app users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg) can override the screen security configuration using `overrideScreenSecurity` and `overrideEditableFlag`.
///
/// This is an [asynchronous operation](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#async). The response object contains a link to the long-running task.
///
/// Note that this operation **only works for issue field select list options added by Connect apps**, it cannot be used with issue field select list options created in Jira or using operations from the [Issue custom field options](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#api-group-Issue-custom-field-options) resource.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). Jira permissions are not required for the app providing the field.
#[derive(Clone)]
pub struct ReplaceIssueFieldOptionRequest<'a> {
    client: &'a crate::core::Client,
    replace_with: Option<i64>,
    jql: Option<String>,
    override_screen_security: Option<bool>,
    override_editable_flag: Option<bool>,
    field_key: String,
    option_id: i64,
}

impl<'a> ReplaceIssueFieldOptionRequest<'a> {
    fn new(client: &'a crate::core::Client, field_key: impl Into<String>, option_id: i64) -> Self {
        Self {
            client,
            field_key: field_key.into(),
            option_id,
            replace_with: None,
            jql: None,
            override_screen_security: None,
            override_editable_flag: None,
        }
    }

    /// The ID of the option that will replace the currently selected option.
    #[must_use]
    pub fn replace_with(mut self, value: i64) -> Self {
        self.replace_with = Some(value);

        self
    }

    /// A JQL query that specifies the issues to be updated. For example, *project=10000*.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// Whether screen security is overridden to enable hidden fields to be edited. Available to Connect and Forge app users with admin permission.
    #[must_use]
    pub fn override_screen_security(mut self, value: bool) -> Self {
        self.override_screen_security = Some(value);

        self
    }

    /// Whether screen security is overridden to enable uneditable fields to be edited. Available to Connect and Forge app users with *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    #[must_use]
    pub fn override_editable_flag(mut self, value: bool) -> Self {
        self.override_editable_flag = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/field/{}/option/{}/issue",
                crate::core::encode_path_segment(&self.field_key),
                self.option_id
            ),
        );

        if let Some(value) = &self.replace_with {
            config.query.push(("replaceWith".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.override_screen_security {
            config
                .query
                .push(("overrideScreenSecurity".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.override_editable_flag {
            config.query.push(("overrideEditableFlag".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<TaskProgressRemoveOptionFromIssuesResult> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
