// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

/// A comma-separated list of the parameters to expand.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetIssuesForSprintRequestExpand {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetIssuesForSprintRequestFields {
    One(String),
    Many(Vec<String>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The Sprint operations.
pub struct SprintService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> SprintService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a future sprint. Sprint name and origin board id are required. Start and end date are optional. Notes: The sprint name is trimmed. Only Jira administrators can create synced sprints.
    pub fn create_sprint(&self, sprint_create: SprintCreate) -> CreateSprintRequest<'a> {
        CreateSprintRequest::new(self.client, sprint_create)
    }

    /// Sets the Synced flag to false for all sprints in the provided list.
    pub fn unmap_sprints(&self, unmap_sprints: UnmapSprints) -> UnmapSprintsRequest<'a> {
        UnmapSprintsRequest::new(self.client, unmap_sprints)
    }

    /// Sets the Synced flag to false for all sprints on this Jira instance. This operation is intended for cleanup only. It is highly destructive and not reversible. Use with caution.
    pub fn unmap_all_sprints(&self) -> UnmapAllSprintsRequest<'a> {
        UnmapAllSprintsRequest::new(self.client)
    }

    /// Returns a single sprint, for a given sprint Id. The sprint will only be returned if the user can view the board that the sprint was created on, or view at least one of the issues in the sprint.
    pub fn get_sprint(&self, sprint_id: i64) -> GetSprintRequest<'a> {
        GetSprintRequest::new(self.client, sprint_id)
    }

    /// Performs a partial update of a sprint.
    /// A partial update means that fields not present in the request JSON will not be updated.
    /// Notes:
    /// - Sprints that are in a closed state cannot be updated.
    /// - A sprint can be started by updating the state to 'active'. This requires the sprint to be in the 'future' state and have a startDate and endDate set.
    /// - A sprint can be completed by updating the state to 'closed'. This action requires the sprint to be in the 'active' state. This sets the completeDate to the time of the request.
    ///   If the sprint has offending issues (those which are complete, but have incomplete subtasks) it cannot be closed.
    ///   If issues are moved to new sprint user has to have issues edit permissions.
    /// - Other changes to state are not allowed.
    /// - The completeDate field cannot be updated manually.
    /// - Sprint goal can be removed by updating it's value to empty string
    /// - Only Jira administrators can edit dates on sprints that are marked as synced.
    pub fn partially_update_sprint(&self, sprint_id: i64, sprint: Sprint) -> PartiallyUpdateSprintRequest<'a> {
        PartiallyUpdateSprintRequest::new(self.client, sprint_id, sprint)
    }

    /// Performs a full update of a sprint.
    /// A full update means that the result will be exactly the same as the request body.
    /// Any fields not present in the request JSON will be set to null.
    /// Notes:
    /// - Sprints that are in a closed state cannot be updated.
    /// - A sprint can be started by updating the state to 'active'. This requires the sprint to be in the 'future' state and have a startDate and endDate set.
    /// - A sprint can be completed by updating the state to 'closed'. This action requires the sprint to be in the 'active' state. This sets the completeDate to the time of the request.
    ///   If the sprint has offending issues (those which are complete, but have incomplete subtasks) it cannot be closed.
    ///   If issues are moved to new sprint user has to have issues edit permissions.
    /// - Other changes to state are not allowed.
    /// - The completeDate field cannot be updated manually.
    /// - Only Jira administrators can edit dates on sprints that are marked as synced.
    pub fn update_sprint(&self, sprint_id: i64, sprint: Sprint) -> UpdateSprintRequest<'a> {
        UpdateSprintRequest::new(self.client, sprint_id, sprint)
    }

    /// Deletes a sprint. Once a sprint is deleted, all issues in the sprint will be moved to the backlog. To delete a synced sprint, you must unsync it first.
    pub fn delete_sprint(&self, sprint_id: i64) -> DeleteSprintRequest<'a> {
        DeleteSprintRequest::new(self.client, sprint_id)
    }

    /// Returns all issues in a sprint, for a given sprint Id. This only includes issues that the user has permission to view. By default, the returned issues are ordered by rank.
    pub fn get_issues_for_sprint(&self, sprint_id: i64) -> GetIssuesForSprintRequest<'a> {
        GetIssuesForSprintRequest::new(self.client, sprint_id)
    }

    /// Moves issues to a sprint, for a given sprint Id. Issues can only be moved to open or active sprints. The maximum number of issues that can be moved in one operation is 50.
    pub fn move_issues_to_sprint(
        &self,
        sprint_id: i64,
        issue_assign_request: IssueAssignRequest,
    ) -> MoveIssuesToSprintRequest<'a> {
        MoveIssuesToSprintRequest::new(self.client, sprint_id, issue_assign_request)
    }

    /// Returns the keys of all properties for the sprint identified by the id. The user who retrieves the property keys is required to have permissions to view the sprint.
    pub fn get_sprint_property_keys(&self, sprint_id: i64) -> GetSprintPropertyKeysRequest<'a> {
        GetSprintPropertyKeysRequest::new(self.client, sprint_id)
    }

    /// Returns the value of the property with a given key from the sprint identified by the provided id. The user who retrieves the property is required to have permissions to view the sprint.
    pub fn get_sprint_property(&self, property_key: impl Into<String>, sprint_id: i64) -> GetSprintPropertyRequest<'a> {
        GetSprintPropertyRequest::new(self.client, property_key, sprint_id)
    }

    /// Sets the value of the specified sprint's property. You can use this resource to store a custom data against the sprint identified by the id. The user who stores the data is required to have permissions to modify the sprint.
    pub fn set_sprint_property(
        &self,
        property_key: impl Into<String>,
        sprint_id: i64,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetSprintPropertyRequest<'a> {
        SetSprintPropertyRequest::new(self.client, property_key, sprint_id, body)
    }

    /// Removes the property from the sprint identified by the id. Ths user removing the property is required to have permissions to modify the sprint.
    pub fn delete_sprint_property(
        &self,
        property_key: impl Into<String>,
        sprint_id: i64,
    ) -> DeleteSprintPropertyRequest<'a> {
        DeleteSprintPropertyRequest::new(self.client, property_key, sprint_id)
    }

    /// Swap the position of the sprint with the second sprint.
    pub fn swap_sprint(&self, sprint_id: i64, sprint_swap: SprintSwap) -> SwapSprintRequest<'a> {
        SwapSprintRequest::new(self.client, sprint_id, sprint_swap)
    }
}

/// Creates a future sprint. Sprint name and origin board id are required. Start and end date are optional. Notes: The sprint name is trimmed. Only Jira administrators can create synced sprints.
pub struct CreateSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_create: SprintCreate,
}

impl<'a> CreateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_create: SprintCreate) -> Self {
        Self { client, sprint_create }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/agile/1.0/sprint".to_owned());

        let body = match serde_json::to_value(&self.sprint_create)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Sprint> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the Synced flag to false for all sprints in the provided list.
pub struct UnmapSprintsRequest<'a> {
    client: &'a crate::core::Client,
    unmap_sprints: UnmapSprints,
}

impl<'a> UnmapSprintsRequest<'a> {
    fn new(client: &'a crate::core::Client, unmap_sprints: UnmapSprints) -> Self {
        Self { client, unmap_sprints }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/agile/1.0/sprint/unmap".to_owned());

        let body = match serde_json::to_value(&self.unmap_sprints)? {
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

/// Sets the Synced flag to false for all sprints on this Jira instance. This operation is intended for cleanup only. It is highly destructive and not reversible. Use with caution.
pub struct UnmapAllSprintsRequest<'a> {
    client: &'a crate::core::Client,
}

impl<'a> UnmapAllSprintsRequest<'a> {
    fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/agile/1.0/sprint/unmap-all".to_owned());

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

/// Returns a single sprint, for a given sprint Id. The sprint will only be returned if the user can view the board that the sprint was created on, or view at least one of the issues in the sprint.
pub struct GetSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
}

impl<'a> GetSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self { client, sprint_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Sprint> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Performs a partial update of a sprint.
/// A partial update means that fields not present in the request JSON will not be updated.
/// Notes:
/// - Sprints that are in a closed state cannot be updated.
/// - A sprint can be started by updating the state to 'active'. This requires the sprint to be in the 'future' state and have a startDate and endDate set.
/// - A sprint can be completed by updating the state to 'closed'. This action requires the sprint to be in the 'active' state. This sets the completeDate to the time of the request.
///   If the sprint has offending issues (those which are complete, but have incomplete subtasks) it cannot be closed.
///   If issues are moved to new sprint user has to have issues edit permissions.
/// - Other changes to state are not allowed.
/// - The completeDate field cannot be updated manually.
/// - Sprint goal can be removed by updating it's value to empty string
/// - Only Jira administrators can edit dates on sprints that are marked as synced.
pub struct PartiallyUpdateSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    sprint: Sprint,
}

impl<'a> PartiallyUpdateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, sprint: Sprint) -> Self {
        Self { client, sprint_id, sprint }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
        );

        let body = match serde_json::to_value(&self.sprint)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Sprint> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Performs a full update of a sprint.
/// A full update means that the result will be exactly the same as the request body.
/// Any fields not present in the request JSON will be set to null.
/// Notes:
/// - Sprints that are in a closed state cannot be updated.
/// - A sprint can be started by updating the state to 'active'. This requires the sprint to be in the 'future' state and have a startDate and endDate set.
/// - A sprint can be completed by updating the state to 'closed'. This action requires the sprint to be in the 'active' state. This sets the completeDate to the time of the request.
///   If the sprint has offending issues (those which are complete, but have incomplete subtasks) it cannot be closed.
///   If issues are moved to new sprint user has to have issues edit permissions.
/// - Other changes to state are not allowed.
/// - The completeDate field cannot be updated manually.
/// - Only Jira administrators can edit dates on sprints that are marked as synced.
pub struct UpdateSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    sprint: Sprint,
}

impl<'a> UpdateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, sprint: Sprint) -> Self {
        Self { client, sprint_id, sprint }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
        );

        let body = match serde_json::to_value(&self.sprint)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Sprint> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes a sprint. Once a sprint is deleted, all issues in the sprint will be moved to the backlog. To delete a synced sprint, you must unsync it first.
pub struct DeleteSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
}

impl<'a> DeleteSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self { client, sprint_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
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

/// Returns all issues in a sprint, for a given sprint Id. This only includes issues that the user has permission to view. By default, the returned issues are ordered by rank.
pub struct GetIssuesForSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    expand: Option<GetIssuesForSprintRequestExpand>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    fields: Option<GetIssuesForSprintRequestFields>,
    start_at: Option<i64>,
}

impl<'a> GetIssuesForSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self {
            client,
            sprint_id,
            expand: None,
            jql: None,
            max_results: None,
            validate_query: None,
            fields: None,
            start_at: None,
        }
    }

    /// A comma-separated list of the parameters to expand.
    #[must_use]
    pub fn expand(mut self, value: GetIssuesForSprintRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// Filters results using a JQL query. If you define an order in your JQL query, it will override the default order of the returned issues.
    #[must_use]
    pub fn jql(mut self, value: impl Into<String>) -> Self {
        self.jql = Some(value.into());

        self
    }

    /// The maximum number of issues to return per page. Default: 50. See the 'Pagination' section at the top of this page for more details. Note, the total number of issues returned is limited by the property 'jira.search.views.default.max' in your JIRA instance. If you exceed this limit, your results will be truncated.
    #[must_use]
    pub fn max_results(mut self, value: i64) -> Self {
        self.max_results = Some(value);

        self
    }

    /// Specifies whether to validate the JQL query or not. Default: true.
    #[must_use]
    pub fn validate_query(mut self, value: bool) -> Self {
        self.validate_query = Some(value);

        self
    }

    /// The list of fields to return for each issue. By default, all navigable and Agile fields are returned.
    #[must_use]
    pub fn fields(mut self, value: GetIssuesForSprintRequestFields) -> Self {
        self.fields = Some(value);

        self
    }

    /// The starting index of the returned issues. Base index: 0. See the 'Pagination' section at the top of this page for more details.
    #[must_use]
    pub fn start_at(mut self, value: i64) -> Self {
        self.start_at = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/sprint/{}/issue", self.sprint_id),
        );

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.jql {
            config.query.push(("jql".to_owned(), crate::core::QueryValue::Scalar(value.clone())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.validate_query {
            config.query.push(("validateQuery".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.fields {
            config.query.push(("fields".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<SearchResults> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Moves issues to a sprint, for a given sprint Id. Issues can only be moved to open or active sprints. The maximum number of issues that can be moved in one operation is 50.
pub struct MoveIssuesToSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    issue_assign_request: IssueAssignRequest,
}

impl<'a> MoveIssuesToSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, issue_assign_request: IssueAssignRequest) -> Self {
        Self { client, sprint_id, issue_assign_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}/issue", self.sprint_id),
        );

        let body = match serde_json::to_value(&self.issue_assign_request)? {
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

/// Returns the keys of all properties for the sprint identified by the id. The user who retrieves the property keys is required to have permissions to view the sprint.
pub struct GetSprintPropertyKeysRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
}

impl<'a> GetSprintPropertyKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self { client, sprint_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/sprint/{}/properties", self.sprint_id),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityPropertiesKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the sprint identified by the provided id. The user who retrieves the property is required to have permissions to view the sprint.
pub struct GetSprintPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    sprint_id: i64,
}

impl<'a> GetSprintPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>, sprint_id: i64) -> Self {
        Self { client, property_key: property_key.into(), sprint_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                self.sprint_id,
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<EntityProperty> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Sets the value of the specified sprint's property. You can use this resource to store a custom data against the sprint identified by the id. The user who stores the data is required to have permissions to modify the sprint.
pub struct SetSprintPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    sprint_id: i64,
    body: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetSprintPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        property_key: impl Into<String>,
        sprint_id: i64,
        body: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, property_key: property_key.into(), sprint_id, body }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                self.sprint_id,
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.body)?));

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

/// Removes the property from the sprint identified by the id. Ths user removing the property is required to have permissions to modify the sprint.
pub struct DeleteSprintPropertyRequest<'a> {
    client: &'a crate::core::Client,
    property_key: String,
    sprint_id: i64,
}

impl<'a> DeleteSprintPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, property_key: impl Into<String>, sprint_id: i64) -> Self {
        Self { client, property_key: property_key.into(), sprint_id }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                self.sprint_id,
                crate::core::encode_path_segment(&self.property_key)
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

/// Swap the position of the sprint with the second sprint.
pub struct SwapSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    sprint_swap: SprintSwap,
}

impl<'a> SwapSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, sprint_swap: SprintSwap) -> Self {
        Self { client, sprint_id, sprint_swap }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}/swap", self.sprint_id),
        );

        let body = match serde_json::to_value(&self.sprint_swap)? {
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
