// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// How to validate the JQL query and treat the validation results. Validation options include:
    ///
    ///  *  `strict` Returns all errors. If validation fails, the query structure is not returned.
    ///  *  `warn` Returns all errors. If validation fails but the JQL query is correctly formed, the query structure is returned.
    ///  *  `none` No validation is performed. If JQL query is correctly formed, the query structure is returned.
    pub enum ParseJqlQueriesRequestValidation {
        Strict => "strict",
        Warn => "warn",
        None => "none",
    }
}

/// The JQL operations.
pub struct JQLService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> JQLService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns reference data for JQL searches. This is a downloadable version of the documentation provided in [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ) and [Advanced searching - functions reference](https://confluence.atlassian.com/x/hgORLQ), along with a list of JQL-reserved words. Use this information to assist with the programmatic creation of JQL queries or the validation of queries built in a custom query builder.
    ///
    /// To filter visible field details by project or collapse non-unique fields by field type then [Get field reference data (POST)](#api-rest-api-3-jql-autocompletedata-post) can be used.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn get_auto_complete(&self) -> GetAutoCompleteRequest<'a> {
        GetAutoCompleteRequest::new(self.client)
    }

    /// Returns reference data for JQL searches. This is a downloadable version of the documentation provided in [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ) and [Advanced searching - functions reference](https://confluence.atlassian.com/x/hgORLQ), along with a list of JQL-reserved words. Use this information to assist with the programmatic creation of JQL queries or the validation of queries built in a custom query builder.
    ///
    /// This operation can filter the custom fields returned by project. Invalid project IDs in `projectIds` are ignored. System fields are always returned.
    ///
    /// It can also return the collapsed field for custom fields. Collapsed fields enable searches to be performed across all fields with the same name and of the same field type. For example, the collapsed field `Component - Component[Dropdown]` enables dropdown fields `Component - cf[10061]` and `Component - cf[10062]` to be searched simultaneously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn get_auto_complete_post(
        &self,
        search_auto_complete_filter: SearchAutoCompleteFilter,
    ) -> GetAutoCompletePostRequest<'a> {
        GetAutoCompletePostRequest::new(self.client, search_auto_complete_filter)
    }

    /// Returns the JQL search auto complete suggestions for a field.
    ///
    /// Suggestions can be obtained by providing:
    ///
    ///  *  `fieldName` to get a list of all values for the field.
    ///  *  `fieldName` and `fieldValue` to get a list of values containing the text in `fieldValue`.
    ///  *  `fieldName` and `predicateName` to get a list of all predicate values for the field.
    ///  *  `fieldName`, `predicateName`, and `predicateValue` to get a list of predicate values containing the text in `predicateValue`.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn get_field_auto_complete_for_query_string(&self) -> GetFieldAutoCompleteForQueryStringRequest<'a> {
        GetFieldAutoCompleteForQueryStringRequest::new(self.client)
    }

    /// Parses and validates JQL queries.
    ///
    /// Validation is performed in context of the current user.
    ///
    /// This operation can be accessed anonymously.
    ///
    /// **[Permissions](#permissions) required:** None.
    pub fn parse_jql_queries(
        &self,
        validation: impl Into<ParseJqlQueriesRequestValidation>,
        jql_queries_to_parse: JqlQueriesToParse,
    ) -> ParseJqlQueriesRequest<'a> {
        ParseJqlQueriesRequest::new(self.client, validation, jql_queries_to_parse)
    }

    /// Converts one or more JQL queries with user identifiers (username or user key) to equivalent JQL queries with account IDs.
    ///
    /// You may wish to use this operation if your system stores JQL queries and you want to make them GDPR-compliant. For more information about GDPR-related changes, see the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/).
    ///
    /// **[Permissions](#permissions) required:** Permission to access Jira.
    pub fn migrate_queries(
        &self,
        jql_personal_data_migration_request: JQLPersonalDataMigrationRequest,
    ) -> MigrateQueriesRequest<'a> {
        MigrateQueriesRequest::new(self.client, jql_personal_data_migration_request)
    }
}

/// Returns reference data for JQL searches. This is a downloadable version of the documentation provided in [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ) and [Advanced searching - functions reference](https://confluence.atlassian.com/x/hgORLQ), along with a list of JQL-reserved words. Use this information to assist with the programmatic creation of JQL queries or the validation of queries built in a custom query builder.
///
/// To filter visible field details by project or collapse non-unique fields by field type then [Get field reference data (POST)](#api-rest-api-3-jql-autocompletedata-post) can be used.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None.
pub struct GetAutoCompleteRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> GetAutoCompleteRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/jql/autocompletedata".to_owned());

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<JQLReferenceData> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns reference data for JQL searches. This is a downloadable version of the documentation provided in [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ) and [Advanced searching - functions reference](https://confluence.atlassian.com/x/hgORLQ), along with a list of JQL-reserved words. Use this information to assist with the programmatic creation of JQL queries or the validation of queries built in a custom query builder.
///
/// This operation can filter the custom fields returned by project. Invalid project IDs in `projectIds` are ignored. System fields are always returned.
///
/// It can also return the collapsed field for custom fields. Collapsed fields enable searches to be performed across all fields with the same name and of the same field type. For example, the collapsed field `Component - Component[Dropdown]` enables dropdown fields `Component - cf[10061]` and `Component - cf[10062]` to be searched simultaneously.
///
/// **[Permissions](#permissions) required:** None.
pub struct GetAutoCompletePostRequest<'a> {
    client: &'a crate::core::Client,
    search_auto_complete_filter: SearchAutoCompleteFilter,
}

impl<'a> GetAutoCompletePostRequest<'a> {
    fn new(client: &'a crate::core::Client, search_auto_complete_filter: SearchAutoCompleteFilter) -> Self {
        Self { client, search_auto_complete_filter }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/jql/autocompletedata".to_owned());

        let body = match serde_json::to_value(&self.search_auto_complete_filter)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<JQLReferenceData> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the JQL search auto complete suggestions for a field.
///
/// Suggestions can be obtained by providing:
///
///  *  `fieldName` to get a list of all values for the field.
///  *  `fieldName` and `fieldValue` to get a list of values containing the text in `fieldValue`.
///  *  `fieldName` and `predicateName` to get a list of all predicate values for the field.
///  *  `fieldName`, `predicateName`, and `predicateValue` to get a list of predicate values containing the text in `predicateValue`.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None.
pub struct GetFieldAutoCompleteForQueryStringRequest<'a> {
    client: &'a crate::core::Client,
    field_name: Option<String>,
    field_value: Option<String>,
    predicate_name: Option<String>,
    predicate_value: Option<String>,
}

impl<'a> GetFieldAutoCompleteForQueryStringRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client, field_name: None, field_value: None, predicate_name: None, predicate_value: None }
    }

    /// The name of the field.
    #[must_use]
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
        self.field_name = Some(value.into());

        self
    }

    /// The partial field item name entered by the user.
    #[must_use]
    pub fn field_value(mut self, value: impl Into<String>) -> Self {
        self.field_value = Some(value.into());

        self
    }

    /// The name of the [ CHANGED operator predicate](https://confluence.atlassian.com/x/hQORLQ#Advancedsearching-operatorsreference-CHANGEDCHANGED) for which the suggestions are generated. The valid predicate operators are *by*, *from*, and *to*.
    #[must_use]
    pub fn predicate_name(mut self, value: impl Into<String>) -> Self {
        self.predicate_name = Some(value.into());

        self
    }

    /// The partial predicate item name entered by the user.
    #[must_use]
    pub fn predicate_value(mut self, value: impl Into<String>) -> Self {
        self.predicate_value = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            "/rest/api/3/jql/autocompletedata/suggestions".to_owned(),
        );

        if let Some(value) = &self.field_name {
            config.query.push(("fieldName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.field_value {
            config.query.push(("fieldValue".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.predicate_name {
            config.query.push(("predicateName".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.predicate_value {
            config.query.push(("predicateValue".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<AutoCompleteSuggestions> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Parses and validates JQL queries.
///
/// Validation is performed in context of the current user.
///
/// This operation can be accessed anonymously.
///
/// **[Permissions](#permissions) required:** None.
pub struct ParseJqlQueriesRequest<'a> {
    client: &'a crate::core::Client,
    validation: ParseJqlQueriesRequestValidation,
    jql_queries_to_parse: JqlQueriesToParse,
}

impl<'a> ParseJqlQueriesRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        validation: impl Into<ParseJqlQueriesRequestValidation>,
        jql_queries_to_parse: JqlQueriesToParse,
    ) -> Self {
        Self { client, validation: validation.into(), jql_queries_to_parse }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/jql/parse".to_owned());

        config.query.push(("validation".to_owned(), crate::core::QueryValue::from_serializable(&self.validation)?));

        let body = match serde_json::to_value(&self.jql_queries_to_parse)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ParsedJqlQueries> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Converts one or more JQL queries with user identifiers (username or user key) to equivalent JQL queries with account IDs.
///
/// You may wish to use this operation if your system stores JQL queries and you want to make them GDPR-compliant. For more information about GDPR-related changes, see the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/).
///
/// **[Permissions](#permissions) required:** Permission to access Jira.
pub struct MigrateQueriesRequest<'a> {
    client: &'a crate::core::Client,
    jql_personal_data_migration_request: JQLPersonalDataMigrationRequest,
}

impl<'a> MigrateQueriesRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        jql_personal_data_migration_request: JQLPersonalDataMigrationRequest,
    ) -> Self {
        Self { client, jql_personal_data_migration_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/api/3/jql/pdcleaner".to_owned());

        let body = match serde_json::to_value(&self.jql_personal_data_migration_request)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<ConvertedJQLQueries> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
