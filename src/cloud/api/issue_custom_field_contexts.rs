// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueCustomFieldContexts operations.
pub struct IssueCustomFieldContextsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueCustomFieldContextsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of [ contexts](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html) for a custom field. Contexts can be returned as follows:
    ///
    ///  *  With no other parameters set, all contexts.
    ///  *  By defining `id` only, all contexts from the list of IDs.
    ///  *  By defining `isAnyIssueType`, limit the list of contexts returned to either those that apply to all issue types (true) or those that apply to only a subset of issue types (false)
    ///  *  By defining `isGlobalContext`, limit the list of contexts return to either those that apply to all projects (global contexts) (true) or those that apply to only a subset of projects (false).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). *Edit Workflow* [edit workflow permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Edit-Workflows)
    pub fn get_contexts_for_field(&self, field_id: impl Into<String>) -> GetContextsForFieldRequest<'a> {
        GetContextsForFieldRequest::new(self.client, field_id)
    }

    /// Creates a custom field context.
    ///
    /// If `projectIds` is empty, a global context is created. A global context is one that applies to all project. If `issueTypeIds` is empty, the context applies to all issue types.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn create_custom_field_context(
        &self,
        field_id: impl Into<String>,
        create_custom_field_context: CreateCustomFieldContext,
    ) -> CreateCustomFieldContextRequest<'a> {
        CreateCustomFieldContextRequest::new(self.client, field_id, create_custom_field_context)
    }

    /// Returns a paginated list of default values grouped by custom field context.
    ///
    /// Each returned `ContextDefaultValuesBean` has a `contextId` and a `defaultValues` list of `IssueTypeDefaultValueBean` entries - one per issue-type-scoped default value configured for the context. An entry with `"isAnyIssueType": true` represents the catch-all default that applies to every issue type covered by the context that is not covered by a more specific entry; a non-null `issueTypeId` represents a default that only applies to that issue type.
    ///
    /// For contexts that have not been converted to the multiple-contexts data model, exactly one entry is returned per context with `isAnyIssueType=true`. For converted contexts, one entry is returned per configured per-issue-type default.
    ///
    /// The value object on each entry is the same polymorphic `CustomFieldContextDefaultValueBean` exposed by the deprecated `GET /defaultValue` endpoint - its concrete subtype depends on the custom field's type (see the list of supported types on that endpoint).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_context_default_values(&self, field_id: impl Into<String>) -> GetContextDefaultValuesRequest<'a> {
        GetContextDefaultValuesRequest::new(self.client, field_id)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of context to issue type mappings for a custom field. Mappings are returned for all contexts or a list of contexts. Mappings are ordered first by context ID and then by issue type ID.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_issue_type_mappings_for_contexts(
        &self,
        field_id: impl Into<String>,
    ) -> GetIssueTypeMappingsForContextsRequest<'a> {
        GetIssueTypeMappingsForContextsRequest::new(self.client, field_id)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of project and issue type mappings and, for each mapping, the ID of a [custom field context](https://confluence.atlassian.com/x/k44fOw) that applies to the project and issue type.
    ///
    /// If there is no custom field context assigned to the project then, if present, the custom field context that applies to all projects is returned if it also applies to the issue type or all issue types. If a custom field context is not found, the returned custom field context ID is `null`.
    ///
    /// Duplicate project and issue type mappings cannot be provided in the request.
    ///
    /// The order of the returned values is the same as provided in the request.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_custom_field_contexts_for_projects_and_issue_types(
        &self,
        field_id: impl Into<String>,
        project_issue_type_mappings: ProjectIssueTypeMappings,
    ) -> GetCustomFieldContextsForProjectsAndIssueTypesRequest<'a> {
        GetCustomFieldContextsForProjectsAndIssueTypesRequest::new(self.client, field_id, project_issue_type_mappings)
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of context to project mappings for a custom field. The result can be filtered by `contextId`. Otherwise, all mappings are returned. Invalid IDs are ignored.
    ///
    /// **Note:** Jira is adding support for multiple field contexts per project. On sites where this is enabled, a custom field can have more than one context associated with the same project, so this operation can return several mappings that share the same `projectId`, each with a different `contextId`. Do not assume that a project appears at most once in the response. See [CHANGE-3082](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3082) for more details.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn get_project_context_mapping(&self, field_id: impl Into<String>) -> GetProjectContextMappingRequest<'a> {
        GetProjectContextMappingRequest::new(self.client, field_id)
    }

    /// Updates a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn update_custom_field_context(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
        custom_field_context_update_details: CustomFieldContextUpdateDetails,
    ) -> UpdateCustomFieldContextRequest<'a> {
        UpdateCustomFieldContextRequest::new(self.client, field_id, context_id, custom_field_context_update_details)
    }

    /// Deletes a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).
    ///
    /// This API will not allow removing the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn delete_custom_field_context(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
    ) -> DeleteCustomFieldContextRequest<'a> {
        DeleteCustomFieldContextRequest::new(self.client, field_id, context_id)
    }

    /// Adds issue types to a custom field context, appending the issue types to the issue types list.
    ///
    /// A custom field context without any issue types applies to all issue types. Adding issue types to such a custom field context would result in it applying to only the listed issue types.
    ///
    /// If any of the issue types exists in the custom field context, the operation fails and no issue types are added.
    ///
    /// This API will not allow adding issue types to the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn add_issue_types_to_context(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
        issue_type_ids: IssueTypeIds,
    ) -> AddIssueTypesToContextRequest<'a> {
        AddIssueTypesToContextRequest::new(self.client, field_id, context_id, issue_type_ids)
    }

    /// Removes issue types from a custom field context.
    ///
    /// A custom field context without any issue types applies to all issue types.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn remove_issue_types_from_context(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
        issue_type_ids: IssueTypeIds,
    ) -> RemoveIssueTypesFromContextRequest<'a> {
        RemoveIssueTypesFromContextRequest::new(self.client, field_id, context_id, issue_type_ids)
    }

    /// Assigns a custom field context to projects.
    ///
    /// If any project in the request is assigned to any context of the custom field, the operation fails.
    ///
    /// This API will not allow adding projects to the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn assign_projects_to_custom_field_context(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
        project_ids: ProjectIds,
    ) -> AssignProjectsToCustomFieldContextRequest<'a> {
        AssignProjectsToCustomFieldContextRequest::new(self.client, field_id, context_id, project_ids)
    }

    /// Removes a custom field context from projects.
    ///
    /// A custom field context without any projects applies to all projects. Removing all projects from a custom field context would result in it applying to all projects.
    ///
    /// If any project in the request is not assigned to the context, or the operation would result in two global contexts for the field, the operation fails.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
    pub fn remove_custom_field_context_from_projects(
        &self,
        field_id: impl Into<String>,
        context_id: i64,
        project_ids: ProjectIds,
    ) -> RemoveCustomFieldContextFromProjectsRequest<'a> {
        RemoveCustomFieldContextFromProjectsRequest::new(self.client, field_id, context_id, project_ids)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of [ contexts](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html) for a custom field. Contexts can be returned as follows:
///
///  *  With no other parameters set, all contexts.
///  *  By defining `id` only, all contexts from the list of IDs.
///  *  By defining `isAnyIssueType`, limit the list of contexts returned to either those that apply to all issue types (true) or those that apply to only a subset of issue types (false)
///  *  By defining `isGlobalContext`, limit the list of contexts return to either those that apply to all projects (global contexts) (true) or those that apply to only a subset of projects (false).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg). *Edit Workflow* [edit workflow permission](https://support.atlassian.com/jira-cloud-administration/docs/permissions-for-company-managed-projects/#Edit-Workflows)
pub struct GetContextsForFieldRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    is_any_issue_type: Option<bool>,
    is_global_context: Option<bool>,
    context_id: Option<Vec<i64>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetContextsForFieldRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self {
            client,
            field_id: field_id.into(),
            is_any_issue_type: None,
            is_global_context: None,
            context_id: None,
            start_at: None,
            max_results: None,
        }
    }

    /// Whether to return contexts that apply to all issue types.
    #[must_use]
    pub fn is_any_issue_type(mut self, value: bool) -> Self {
        self.is_any_issue_type = Some(value);

        self
    }

    /// Whether to return contexts that apply to all projects.
    #[must_use]
    pub fn is_global_context(mut self, value: bool) -> Self {
        self.is_global_context = Some(value);

        self
    }

    /// The list of context IDs. To include multiple contexts, separate IDs with ampersand: `contextId=10000&contextId=10001`.
    #[must_use]
    pub fn context_id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.context_id = Some(value.into_iter().collect());

        self
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
            format!("/rest/api/3/field/{}/context", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.is_any_issue_type {
            config.query.push(("isAnyIssueType".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.is_global_context {
            config.query.push(("isGlobalContext".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.context_id {
            config.query.push(("contextId".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomFieldContext>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a custom field context.
///
/// If `projectIds` is empty, a global context is created. A global context is one that applies to all project. If `issueTypeIds` is empty, the context applies to all issue types.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct CreateCustomFieldContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    create_custom_field_context: CreateCustomFieldContext,
}

impl<'a> CreateCustomFieldContextRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        create_custom_field_context: CreateCustomFieldContext,
    ) -> Self {
        Self { client, field_id: field_id.into(), create_custom_field_context }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/api/3/field/{}/context", crate::core::encode_path_segment(&self.field_id)),
        );

        let body = match serde_json::to_value(&self.create_custom_field_context)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CreateCustomFieldContext> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a paginated list of default values grouped by custom field context.
///
/// Each returned `ContextDefaultValuesBean` has a `contextId` and a `defaultValues` list of `IssueTypeDefaultValueBean` entries - one per issue-type-scoped default value configured for the context. An entry with `"isAnyIssueType": true` represents the catch-all default that applies to every issue type covered by the context that is not covered by a more specific entry; a non-null `issueTypeId` represents a default that only applies to that issue type.
///
/// For contexts that have not been converted to the multiple-contexts data model, exactly one entry is returned per context with `isAnyIssueType=true`. For converted contexts, one entry is returned per configured per-issue-type default.
///
/// The value object on each entry is the same polymorphic `CustomFieldContextDefaultValueBean` exposed by the deprecated `GET /defaultValue` endpoint - its concrete subtype depends on the custom field's type (see the list of supported types on that endpoint).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetContextDefaultValuesRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: Option<Vec<i64>>,
    issue_type_id: Option<Vec<String>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetContextDefaultValuesRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self {
            client,
            field_id: field_id.into(),
            context_id: None,
            issue_type_id: None,
            start_at: None,
            max_results: None,
        }
    }

    /// The IDs of the contexts to return default values for. If omitted, default values for every context the custom field has are returned.
    #[must_use]
    pub fn context_id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.context_id = Some(value.into_iter().collect());

        self
    }

    /// The IDs of the issue types to restrict the returned per-issue-type default values to. If omitted, default values for every issue type are returned. This filter never removes the catch-all `isAnyIssueType` entry of a context.
    #[must_use]
    pub fn issue_type_id(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.issue_type_id = Some(value.into_iter().map(Into::into).collect());

        self
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
            format!("/rest/api/3/field/{}/context/defaultValues", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.context_id {
            config.query.push(("contextId".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.issue_type_id {
            config.query.push(("issueTypeId".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ContextDefaultValues>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of context to issue type mappings for a custom field. Mappings are returned for all contexts or a list of contexts. Mappings are ordered first by context ID and then by issue type ID.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetIssueTypeMappingsForContextsRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: Option<Vec<i64>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetIssueTypeMappingsForContextsRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self { client, field_id: field_id.into(), context_id: None, start_at: None, max_results: None }
    }

    /// The ID of the context. To include multiple contexts, provide an ampersand-separated list. For example, `contextId=10001&contextId=10002`.
    #[must_use]
    pub fn context_id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.context_id = Some(value.into_iter().collect());

        self
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
            format!("/rest/api/3/field/{}/context/issuetypemapping", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.context_id {
            config.query.push(("contextId".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<IssueTypeToContextMapping>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of project and issue type mappings and, for each mapping, the ID of a [custom field context](https://confluence.atlassian.com/x/k44fOw) that applies to the project and issue type.
///
/// If there is no custom field context assigned to the project then, if present, the custom field context that applies to all projects is returned if it also applies to the issue type or all issue types. If a custom field context is not found, the returned custom field context ID is `null`.
///
/// Duplicate project and issue type mappings cannot be provided in the request.
///
/// The order of the returned values is the same as provided in the request.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetCustomFieldContextsForProjectsAndIssueTypesRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    start_at: Option<i64>,
    max_results: Option<i64>,
    project_issue_type_mappings: ProjectIssueTypeMappings,
}

impl<'a> GetCustomFieldContextsForProjectsAndIssueTypesRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        project_issue_type_mappings: ProjectIssueTypeMappings,
    ) -> Self {
        Self { client, field_id: field_id.into(), project_issue_type_mappings, start_at: None, max_results: None }
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
            crate::core::Method::POST,
            format!("/rest/api/3/field/{}/context/mapping", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.project_issue_type_mappings)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<ContextForProjectAndIssueType>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of context to project mappings for a custom field. The result can be filtered by `contextId`. Otherwise, all mappings are returned. Invalid IDs are ignored.
///
/// **Note:** Jira is adding support for multiple field contexts per project. On sites where this is enabled, a custom field can have more than one context associated with the same project, so this operation can return several mappings that share the same `projectId`, each with a different `contextId`. Do not assume that a project appears at most once in the response. See [CHANGE-3082](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3082) for more details.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct GetProjectContextMappingRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: Option<Vec<i64>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
}

impl<'a> GetProjectContextMappingRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>) -> Self {
        Self { client, field_id: field_id.into(), context_id: None, start_at: None, max_results: None }
    }

    /// The list of context IDs. To include multiple context, separate IDs with ampersand: `contextId=10000&contextId=10001`.
    #[must_use]
    pub fn context_id(mut self, value: impl IntoIterator<Item = i64>) -> Self {
        self.context_id = Some(value.into_iter().collect());

        self
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
            format!("/rest/api/3/field/{}/context/projectmapping", crate::core::encode_path_segment(&self.field_id)),
        );

        if let Some(value) = &self.context_id {
            config.query.push(("contextId".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<CustomFieldContextProjectMapping>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct UpdateCustomFieldContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
    custom_field_context_update_details: CustomFieldContextUpdateDetails,
}

impl<'a> UpdateCustomFieldContextRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        context_id: i64,
        custom_field_context_update_details: CustomFieldContextUpdateDetails,
    ) -> Self {
        Self { client, field_id: field_id.into(), context_id, custom_field_context_update_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/field/{}/context/{}",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
            ),
        );

        let body = match serde_json::to_value(&self.custom_field_context_update_details)? {
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

/// Deletes a [ custom field context](https://confluence.atlassian.com/adminjiracloud/what-are-custom-field-contexts-991923859.html).
///
/// This API will not allow removing the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct DeleteCustomFieldContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
}

impl<'a> DeleteCustomFieldContextRequest<'a> {
    fn new(client: &'a crate::core::Client, field_id: impl Into<String>, context_id: i64) -> Self {
        Self { client, field_id: field_id.into(), context_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/api/3/field/{}/context/{}",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
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

/// Adds issue types to a custom field context, appending the issue types to the issue types list.
///
/// A custom field context without any issue types applies to all issue types. Adding issue types to such a custom field context would result in it applying to only the listed issue types.
///
/// If any of the issue types exists in the custom field context, the operation fails and no issue types are added.
///
/// This API will not allow adding issue types to the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct AddIssueTypesToContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
    issue_type_ids: IssueTypeIds,
}

impl<'a> AddIssueTypesToContextRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        context_id: i64,
        issue_type_ids: IssueTypeIds,
    ) -> Self {
        Self { client, field_id: field_id.into(), context_id, issue_type_ids }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/field/{}/context/{}/issuetype",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
            ),
        );

        let body = match serde_json::to_value(&self.issue_type_ids)? {
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

/// Removes issue types from a custom field context.
///
/// A custom field context without any issue types applies to all issue types.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct RemoveIssueTypesFromContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
    issue_type_ids: IssueTypeIds,
}

impl<'a> RemoveIssueTypesFromContextRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        context_id: i64,
        issue_type_ids: IssueTypeIds,
    ) -> Self {
        Self { client, field_id: field_id.into(), context_id, issue_type_ids }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/api/3/field/{}/context/{}/issuetype/remove",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
            ),
        );

        let body = match serde_json::to_value(&self.issue_type_ids)? {
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

/// Assigns a custom field context to projects.
///
/// If any project in the request is assigned to any context of the custom field, the operation fails.
///
/// This API will not allow adding projects to the global context from April 2026. Instead, an HTTP 400 response will be returned. See [CHANGE-3019](https://developer.atlassian.com/changelog/#CHANGE-3019)
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct AssignProjectsToCustomFieldContextRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
    project_ids: ProjectIds,
}

impl<'a> AssignProjectsToCustomFieldContextRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        context_id: i64,
        project_ids: ProjectIds,
    ) -> Self {
        Self { client, field_id: field_id.into(), context_id, project_ids }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/api/3/field/{}/context/{}/project",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
            ),
        );

        let body = match serde_json::to_value(&self.project_ids)? {
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

/// Removes a custom field context from projects.
///
/// A custom field context without any projects applies to all projects. Removing all projects from a custom field context would result in it applying to all projects.
///
/// If any project in the request is not assigned to the context, or the operation would result in two global contexts for the field, the operation fails.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).
pub struct RemoveCustomFieldContextFromProjectsRequest<'a> {
    client: &'a crate::core::Client,
    field_id: String,
    context_id: i64,
    project_ids: ProjectIds,
}

impl<'a> RemoveCustomFieldContextFromProjectsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        field_id: impl Into<String>,
        context_id: i64,
        project_ids: ProjectIds,
    ) -> Self {
        Self { client, field_id: field_id.into(), context_id, project_ids }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!(
                "/rest/api/3/field/{}/context/{}/project/remove",
                crate::core::encode_path_segment(&self.field_id),
                self.context_id
            ),
        );

        let body = match serde_json::to_value(&self.project_ids)? {
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
