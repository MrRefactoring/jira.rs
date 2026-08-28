// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The IssueFields operations.
pub struct IssueFieldsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> IssueFieldsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a list of Custom Fields in the given range.
    pub fn get_custom_fields(&self) -> GetCustomFieldsRequest<'a> {
        GetCustomFieldsRequest::new(self.client)
    }

    /// Deletes custom fields in bulk.
    pub fn bulk_delete_custom_fields(&self, ids: impl Into<String>) -> BulkDeleteCustomFieldsRequest<'a> {
        BulkDeleteCustomFieldsRequest::new(self.client, ids)
    }

    /// Returns custom field's options defined in a given context composed of projects and issue types.
    pub fn get_custom_field_options(&self, custom_field_id: impl Into<String>) -> GetCustomFieldOptionsRequest<'a> {
        GetCustomFieldOptionsRequest::new(self.client, custom_field_id)
    }

    /// Returns a list of all fields, both System and Custom
    pub fn get_fields(&self) -> GetFieldsRequest<'a> {
        GetFieldsRequest::new(self.client)
    }

    /// Creates a custom field using a definition
    pub fn create_custom_field(&self) -> CreateCustomFieldRequest<'a> {
        CreateCustomFieldRequest::new(self.client)
    }
}

/// Returns a list of Custom Fields in the given range.
pub struct GetCustomFieldsRequest<'a> {
    client: &'a crate::core::Client,
    sort_column: Option<String>,
    types: Option<String>,
    search: Option<String>,
    max_results: Option<String>,
    sort_order: Option<String>,
    screen_ids: Option<String>,
    last_value_update: Option<String>,
    project_ids: Option<String>,
    start_at: Option<String>,
}

impl<'a> GetCustomFieldsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self {
            client,
            sort_column: None,
            types: None,
            search: None,
            max_results: None,
            sort_order: None,
            screen_ids: None,
            last_value_update: None,
            project_ids: None,
            start_at: None,
        }
    }

    /// The column by which to sort the returned custom fields.
    #[must_use]
    pub fn sort_column(mut self, value: impl Into<String>) -> Self {
        self.sort_column = Some(value.into());

        self
    }

    /// A list of custom field types to filter the custom fields.
    #[must_use]
    pub fn types(mut self, value: impl Into<String>) -> Self {
        self.types = Some(value.into());

        self
    }

    /// A query string used to search custom fields.
    #[must_use]
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());

        self
    }

    /// The maximum number of custom fields to return.
    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    /// The order in which to sort the returned custom fields.
    #[must_use]
    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());

        self
    }

    /// A list of screen IDs to filter the custom fields.
    #[must_use]
    pub fn screen_ids(mut self, value: impl Into<String>) -> Self {
        self.screen_ids = Some(value.into());

        self
    }

    /// The last value update to filter the custom fields.
    #[must_use]
    pub fn last_value_update(mut self, value: impl Into<String>) -> Self {
        self.last_value_update = Some(value.into());

        self
    }

    /// A list of project IDs to filter the custom fields.
    #[must_use]
    pub fn project_ids(mut self, value: impl Into<String>) -> Self {
        self.project_ids = Some(value.into());

        self
    }

    /// The starting index of the returned custom fields.
    #[must_use]
    pub fn start_at(mut self, value: impl Into<String>) -> Self {
        self.start_at = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/customFields".to_owned());

        if let Some(value) = &self.sort_column {
            config.query.push(("sortColumn".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.types {
            config.query.push(("types".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.search {
            config.query.push(("search".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.sort_order {
            config.query.push(("sortOrder".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.screen_ids {
            config.query.push(("screenIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.last_value_update {
            config.query.push(("lastValueUpdate".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_ids {
            config.query.push(("projectIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CustomField> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes custom fields in bulk.
pub struct BulkDeleteCustomFieldsRequest<'a> {
    client: &'a crate::core::Client,
    ids: String,
}

impl<'a> BulkDeleteCustomFieldsRequest<'a> {
    fn new(client: &'a crate::core::Client, ids: impl Into<String>) -> Self {
        Self { client, ids: ids.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::DELETE, "/rest/api/2/customFields".to_owned());

        config.query.push(("ids".to_owned(), crate::core::QueryValue::Scalar(self.ids.clone())));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<BulkDeleteResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns custom field's options defined in a given context composed of projects and issue types.
pub struct GetCustomFieldOptionsRequest<'a> {
    client: &'a crate::core::Client,
    max_results: Option<String>,
    issue_type_ids: Option<String>,
    query: Option<String>,
    sort_by_option_name: Option<String>,
    custom_field_id: String,
    use_all_contexts: Option<String>,
    page: Option<String>,
    project_ids: Option<String>,
}

impl<'a> GetCustomFieldOptionsRequest<'a> {
    fn new(client: &'a crate::core::Client, custom_field_id: impl Into<String>) -> Self {
        Self {
            client,
            custom_field_id: custom_field_id.into(),
            max_results: None,
            issue_type_ids: None,
            query: None,
            sort_by_option_name: None,
            use_all_contexts: None,
            page: None,
            project_ids: None,
        }
    }

    /// The maximum number of results to return.
    #[must_use]
    pub fn max_results(mut self, value: impl Into<String>) -> Self {
        self.max_results = Some(value.into());

        self
    }

    /// A list of issue type IDs in a context.
    #[must_use]
    pub fn issue_type_ids(mut self, value: impl Into<String>) -> Self {
        self.issue_type_ids = Some(value.into());

        self
    }

    /// A string used to filter options.
    #[must_use]
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());

        self
    }

    /// Flag to sort options by their names.
    #[must_use]
    pub fn sort_by_option_name(mut self, value: impl Into<String>) -> Self {
        self.sort_by_option_name = Some(value.into());

        self
    }

    /// Flag to fetch all options regardless of context, project IDs, or issue type IDs.
    #[must_use]
    pub fn use_all_contexts(mut self, value: impl Into<String>) -> Self {
        self.use_all_contexts = Some(value.into());

        self
    }

    /// The page of options to return.
    #[must_use]
    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());

        self
    }

    /// A list of project IDs in a context.
    #[must_use]
    pub fn project_ids(mut self, value: impl Into<String>) -> Self {
        self.project_ids = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/api/2/customFields/{}/options", crate::core::encode_path_segment(&self.custom_field_id)),
        );

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.issue_type_ids {
            config.query.push(("issueTypeIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.query {
            config.query.push(("query".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.sort_by_option_name {
            config.query.push(("sortByOptionName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.use_all_contexts {
            config.query.push(("useAllContexts".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.page {
            config.query.push(("page".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.project_ids {
            config.query.push(("projectIds".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<CustomFieldOptions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns a list of all fields, both System and Custom
pub struct GetFieldsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetFieldsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/2/field".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Vec<Field>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Creates a custom field using a definition
pub struct CreateCustomFieldRequest<'a> {
    client: &'a crate::core::Client,
    custom_field_definition_json: Option<CustomFieldDefinitionJson>,
}

impl<'a> CreateCustomFieldRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, custom_field_definition_json: None }
    }

    #[must_use]
    pub fn custom_field_definition_json(mut self, value: CustomFieldDefinitionJson) -> Self {
        self.custom_field_definition_json = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/2/field".to_owned());

        let body = match serde_json::to_value(&self.custom_field_definition_json)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Field> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
