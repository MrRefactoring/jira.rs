// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `functionKey` Sorts by the functionKey.
    ///  *  `used` Sorts by the used timestamp.
    ///  *  `created` Sorts by the created timestamp.
    ///  *  `updated` Sorts by the updated timestamp.
    pub enum GetPrecomputationsRequestOrderBy {
        FunctionKey => "functionKey",
        FunctionKeyDescending => "-functionKey",
        FunctionKeyAscending => "+functionKey",
        Used => "used",
        UsedDescending => "-used",
        UsedAscending => "+used",
        Created => "created",
        CreatedDescending => "-created",
        CreatedAscending => "+created",
        Updated => "updated",
        UpdatedDescending => "-updated",
        UpdatedAscending => "+updated",
    }
}

crate::open_enum! {
    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `functionKey` Sorts by the functionKey.
    ///  *  `used` Sorts by the used timestamp.
    ///  *  `created` Sorts by the created timestamp.
    ///  *  `updated` Sorts by the updated timestamp.
    pub enum GetPrecomputationsByIDRequestOrderBy {
        FunctionKey => "functionKey",
        FunctionKeyDescending => "-functionKey",
        FunctionKeyAscending => "+functionKey",
        Used => "used",
        UsedDescending => "-used",
        UsedAscending => "+used",
        Created => "created",
        CreatedDescending => "-created",
        CreatedAscending => "+created",
        Updated => "updated",
        UpdatedDescending => "-updated",
        UpdatedAscending => "+updated",
    }
}

/// The JQLFunctionsApps operations.
pub struct JQLFunctionsAppsService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> JQLFunctionsAppsService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns the list of a function's precomputations along with information about when they were created, updated, and last used. Each precomputation has a `value` \- the JQL fragment to replace the custom function clause with.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.
    ///
    /// The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
    pub fn get_precomputations(&self) -> GetPrecomputationsRequest<'a> {
        GetPrecomputationsRequest::new(self.client)
    }

    /// Update the precomputation value of a function created by a Forge/Connect app.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** An API for apps to update their own precomputations.
    ///
    /// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
    pub fn update_precomputations(
        &self,
        jql_function_precomputation_update_request: JqlFunctionPrecomputationUpdateRequest,
    ) -> UpdatePrecomputationsRequest<'a> {
        UpdatePrecomputationsRequest::new(self.client, jql_function_precomputation_update_request)
    }

    /// Returns function precomputations by IDs, along with information about when they were created, updated, and last used. Each precomputation has a `value` \- the JQL fragment to replace the custom function clause with.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.
    ///
    /// The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
    pub fn get_precomputations_by_id(
        &self,
        jql_function_precomputation_get_by_id_request: JqlFunctionPrecomputationGetByIdRequest,
    ) -> GetPrecomputationsByIDRequest<'a> {
        GetPrecomputationsByIDRequest::new(self.client, jql_function_precomputation_get_by_id_request)
    }
}

/// Returns the list of a function's precomputations along with information about when they were created, updated, and last used. Each precomputation has a `value` \- the JQL fragment to replace the custom function clause with.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.
///
/// The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
#[derive(Clone)]
pub struct GetPrecomputationsRequest<'a> {
    client: &'a crate::core::Client,
    function_key: Option<Vec<String>>,
    start_at: Option<i64>,
    max_results: Option<i64>,
    order_by: Option<GetPrecomputationsRequestOrderBy>,
}

impl<'a> GetPrecomputationsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, function_key: None, start_at: None, max_results: None, order_by: None }
    }

    /// The function key in format:
    ///
    ///  *  Forge: `ari:cloud:ecosystem::extension/[App ID]/[Environment ID]/static/[Function key from manifest]`
    ///  *  Connect: `[App key]__[Module key]`
    #[must_use]
    pub fn function_key(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.function_key = Some(value.into_iter().map(Into::into).collect());

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

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `functionKey` Sorts by the functionKey.
    ///  *  `used` Sorts by the used timestamp.
    ///  *  `created` Sorts by the created timestamp.
    ///  *  `updated` Sorts by the updated timestamp.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetPrecomputationsRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/jql/function/computation".to_owned(),
        );

        if let Some(value) = &self.function_key {
            config.query.push(("functionKey".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<JqlFunctionPrecomputation>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Update the precomputation value of a function created by a Forge/Connect app.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** An API for apps to update their own precomputations.
///
/// The new `write:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
#[derive(Clone)]
pub struct UpdatePrecomputationsRequest<'a> {
    client: &'a crate::core::Client,
    skip_not_found_precomputations: Option<bool>,
    jql_function_precomputation_update_request: JqlFunctionPrecomputationUpdateRequest,
}

impl<'a> UpdatePrecomputationsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        jql_function_precomputation_update_request: JqlFunctionPrecomputationUpdateRequest,
    ) -> Self {
        Self { client, jql_function_precomputation_update_request, skip_not_found_precomputations: None }
    }

    #[must_use]
    pub fn skip_not_found_precomputations(mut self, value: bool) -> Self {
        self.skip_not_found_precomputations = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/3/jql/function/computation".to_owned(),
        );

        if let Some(value) = &self.skip_not_found_precomputations {
            config
                .query
                .push(("skipNotFoundPrecomputations".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        let body = match serde_json::to_value(&self.jql_function_precomputation_update_request)? {
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

/// Returns function precomputations by IDs, along with information about when they were created, updated, and last used. Each precomputation has a `value` \- the JQL fragment to replace the custom function clause with.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** This API is only accessible to apps and apps can only inspect their own functions.
///
/// The new `read:app-data:jira` OAuth scope is 100% optional now, and not using it won't break your app. However, we recommend adding it to your app's scope list because we will eventually make it mandatory.
#[derive(Clone)]
pub struct GetPrecomputationsByIDRequest<'a> {
    client: &'a crate::core::Client,
    order_by: Option<GetPrecomputationsByIDRequestOrderBy>,
    jql_function_precomputation_get_by_id_request: JqlFunctionPrecomputationGetByIdRequest,
}

impl<'a> GetPrecomputationsByIDRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        jql_function_precomputation_get_by_id_request: JqlFunctionPrecomputationGetByIdRequest,
    ) -> Self {
        Self { client, jql_function_precomputation_get_by_id_request, order_by: None }
    }

    /// [Order](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#ordering) the results by a field:
    ///
    ///  *  `functionKey` Sorts by the functionKey.
    ///  *  `used` Sorts by the used timestamp.
    ///  *  `created` Sorts by the created timestamp.
    ///  *  `updated` Sorts by the updated timestamp.
    #[must_use]
    pub fn order_by(mut self, value: impl Into<GetPrecomputationsByIDRequestOrderBy>) -> Self {
        self.order_by = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/api/3/jql/function/computation/search".to_owned(),
        );

        if let Some(value) = &self.order_by {
            config.query.push(("orderBy".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        let body = match serde_json::to_value(&self.jql_function_precomputation_get_by_id_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<JqlFunctionPrecomputationGetByIdResponse> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
