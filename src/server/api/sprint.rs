// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

/// The Sprint operations.
pub struct SprintService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> SprintService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Creates a future sprint. Sprint name and origin board id are required. Start and end date are optional. Notes: The sprint name is trimmed. Only Jira administrators can create synced sprints.
    pub fn create_sprint(&self, name: impl Into<String>) -> CreateSprintRequest<'a> {
        CreateSprintRequest::new(self.client, name)
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
    pub fn partially_update_sprint(&self, sprint_id: i64) -> PartiallyUpdateSprintRequest<'a> {
        PartiallyUpdateSprintRequest::new(self.client, sprint_id)
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
    pub fn update_sprint(
        &self,
        sprint_id: i64,
        name: impl Into<String>,
        state: impl Into<String>,
    ) -> UpdateSprintRequest<'a> {
        UpdateSprintRequest::new(self.client, sprint_id, name, state)
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
    pub fn move_issues_to_sprint_and_rank(
        &self,
        sprint_id: i64,
        issue_rank_request: IssueAssignRequest,
    ) -> MoveIssuesToSprintAndRankRequest<'a> {
        MoveIssuesToSprintAndRankRequest::new(self.client, sprint_id, issue_rank_request)
    }

    /// Returns the keys of all properties for the sprint identified by the id. The user who retrieves the property keys is required to have permissions to view the sprint.
    pub fn get_properties_keys(&self, sprint_id: impl Into<String>) -> GetPropertiesKeysRequest<'a> {
        GetPropertiesKeysRequest::new(self.client, sprint_id)
    }

    /// Returns the value of the property with a given key from the sprint identified by the provided id. The user who retrieves the property is required to have permissions to view the sprint.
    pub fn get_property(
        &self,
        sprint_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> GetPropertyRequest<'a> {
        GetPropertyRequest::new(self.client, sprint_id, property_key)
    }

    /// Sets the value of the specified sprint's property. You can use this resource to store a custom data against the sprint identified by the id. The user who stores the data is required to have permissions to modify the sprint.
    pub fn set_property(
        &self,
        sprint_id: impl Into<String>,
        property_key: impl Into<String>,
        property_value: std::collections::HashMap<String, serde_json::Value>,
    ) -> SetPropertyRequest<'a> {
        SetPropertyRequest::new(self.client, sprint_id, property_key, property_value)
    }

    /// Removes the property from the sprint identified by the id. Ths user removing the property is required to have permissions to modify the sprint.
    pub fn delete_property(
        &self,
        sprint_id: impl Into<String>,
        property_key: impl Into<String>,
    ) -> DeletePropertyRequest<'a> {
        DeletePropertyRequest::new(self.client, sprint_id, property_key)
    }

    /// Swap the position of the sprint with the second sprint.
    pub fn swap_sprint(&self, sprint_id: i64) -> SwapSprintRequest<'a> {
        SwapSprintRequest::new(self.client, sprint_id)
    }
}

/// Creates a future sprint. Sprint name and origin board id are required. Start and end date are optional. Notes: The sprint name is trimmed. Only Jira administrators can create synced sprints.
#[derive(Clone)]
pub struct CreateSprintRequest<'a> {
    client: &'a crate::core::Client,
    auto_start_stop: Option<bool>,
    end_date: Option<String>,
    goal: Option<String>,
    incomplete_issues_destination_id: Option<i64>,
    name: String,
    origin_board_id: Option<i64>,
    start_date: Option<String>,
    synced: Option<bool>,
    user_profile_time_zone: Option<String>,
}

impl<'a> CreateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, name: impl Into<String>) -> Self {
        Self {
            client,
            name: name.into(),
            auto_start_stop: None,
            end_date: None,
            goal: None,
            incomplete_issues_destination_id: None,
            origin_board_id: None,
            start_date: None,
            synced: None,
            user_profile_time_zone: None,
        }
    }

    #[must_use]
    pub fn auto_start_stop(mut self, value: bool) -> Self {
        self.auto_start_stop = Some(value);

        self
    }

    #[must_use]
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn goal(mut self, value: impl Into<String>) -> Self {
        self.goal = Some(value.into());

        self
    }

    #[must_use]
    pub fn incomplete_issues_destination_id(mut self, value: i64) -> Self {
        self.incomplete_issues_destination_id = Some(value);

        self
    }

    #[must_use]
    pub fn origin_board_id(mut self, value: i64) -> Self {
        self.origin_board_id = Some(value);

        self
    }

    #[must_use]
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn synced(mut self, value: bool) -> Self {
        self.synced = Some(value);

        self
    }

    #[must_use]
    pub fn user_profile_time_zone(mut self, value: impl Into<String>) -> Self {
        self.user_profile_time_zone = Some(value.into());

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::POST, "/rest/agile/1.0/sprint".to_owned());

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.auto_start_stop {
            body.insert("autoStartStop".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.end_date {
            body.insert("endDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.goal {
            body.insert("goal".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.incomplete_issues_destination_id {
            body.insert("incompleteIssuesDestinationId".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("name".to_owned(), serde_json::to_value(&self.name)?);

        if let Some(value) = &self.origin_board_id {
            body.insert("originBoardId".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.start_date {
            body.insert("startDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.synced {
            body.insert("synced".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.user_profile_time_zone {
            body.insert("userProfileTimeZone".to_owned(), serde_json::to_value(value)?);
        }

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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct PartiallyUpdateSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    activated_date: Option<String>,
    auto_start_stop: Option<bool>,
    complete_date: Option<String>,
    end_date: Option<String>,
    goal: Option<String>,
    id: Option<i64>,
    incomplete_issues_destination_id: Option<i64>,
    name: Option<String>,
    origin_board_id: Option<i64>,
    self_: Option<String>,
    start_date: Option<String>,
    state: Option<String>,
    synced: Option<bool>,
}

impl<'a> PartiallyUpdateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self {
            client,
            sprint_id,
            activated_date: None,
            auto_start_stop: None,
            complete_date: None,
            end_date: None,
            goal: None,
            id: None,
            incomplete_issues_destination_id: None,
            name: None,
            origin_board_id: None,
            self_: None,
            start_date: None,
            state: None,
            synced: None,
        }
    }

    #[must_use]
    pub fn activated_date(mut self, value: impl Into<String>) -> Self {
        self.activated_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn auto_start_stop(mut self, value: bool) -> Self {
        self.auto_start_stop = Some(value);

        self
    }

    #[must_use]
    pub fn complete_date(mut self, value: impl Into<String>) -> Self {
        self.complete_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn goal(mut self, value: impl Into<String>) -> Self {
        self.goal = Some(value.into());

        self
    }

    #[must_use]
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);

        self
    }

    #[must_use]
    pub fn incomplete_issues_destination_id(mut self, value: i64) -> Self {
        self.incomplete_issues_destination_id = Some(value);

        self
    }

    #[must_use]
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());

        self
    }

    #[must_use]
    pub fn origin_board_id(mut self, value: i64) -> Self {
        self.origin_board_id = Some(value);

        self
    }

    #[must_use]
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());

        self
    }

    #[must_use]
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());

        self
    }

    #[must_use]
    pub fn synced(mut self, value: bool) -> Self {
        self.synced = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.activated_date {
            body.insert("activatedDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.auto_start_stop {
            body.insert("autoStartStop".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.complete_date {
            body.insert("completeDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.end_date {
            body.insert("endDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.goal {
            body.insert("goal".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.id {
            body.insert("id".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.incomplete_issues_destination_id {
            body.insert("incompleteIssuesDestinationId".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.name {
            body.insert("name".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.origin_board_id {
            body.insert("originBoardId".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.self_ {
            body.insert("self".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.start_date {
            body.insert("startDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.state {
            body.insert("state".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.synced {
            body.insert("synced".to_owned(), serde_json::to_value(value)?);
        }

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
#[derive(Clone)]
pub struct UpdateSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    activated_date: Option<String>,
    auto_start_stop: Option<bool>,
    complete_date: Option<String>,
    end_date: Option<String>,
    goal: Option<String>,
    id: Option<i64>,
    incomplete_issues_destination_id: Option<i64>,
    name: String,
    origin_board_id: Option<i64>,
    self_: Option<String>,
    start_date: Option<String>,
    state: String,
    synced: Option<bool>,
}

impl<'a> UpdateSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, name: impl Into<String>, state: impl Into<String>) -> Self {
        Self {
            client,
            sprint_id,
            name: name.into(),
            state: state.into(),
            activated_date: None,
            auto_start_stop: None,
            complete_date: None,
            end_date: None,
            goal: None,
            id: None,
            incomplete_issues_destination_id: None,
            origin_board_id: None,
            self_: None,
            start_date: None,
            synced: None,
        }
    }

    #[must_use]
    pub fn activated_date(mut self, value: impl Into<String>) -> Self {
        self.activated_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn auto_start_stop(mut self, value: bool) -> Self {
        self.auto_start_stop = Some(value);

        self
    }

    #[must_use]
    pub fn complete_date(mut self, value: impl Into<String>) -> Self {
        self.complete_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn goal(mut self, value: impl Into<String>) -> Self {
        self.goal = Some(value.into());

        self
    }

    #[must_use]
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);

        self
    }

    #[must_use]
    pub fn incomplete_issues_destination_id(mut self, value: i64) -> Self {
        self.incomplete_issues_destination_id = Some(value);

        self
    }

    #[must_use]
    pub fn origin_board_id(mut self, value: i64) -> Self {
        self.origin_board_id = Some(value);

        self
    }

    #[must_use]
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());

        self
    }

    #[must_use]
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());

        self
    }

    #[must_use]
    pub fn synced(mut self, value: bool) -> Self {
        self.synced = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/agile/1.0/sprint/{}", self.sprint_id),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.activated_date {
            body.insert("activatedDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.auto_start_stop {
            body.insert("autoStartStop".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.complete_date {
            body.insert("completeDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.end_date {
            body.insert("endDate".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.goal {
            body.insert("goal".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.id {
            body.insert("id".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.incomplete_issues_destination_id {
            body.insert("incompleteIssuesDestinationId".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("name".to_owned(), serde_json::to_value(&self.name)?);

        if let Some(value) = &self.origin_board_id {
            body.insert("originBoardId".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.self_ {
            body.insert("self".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.start_date {
            body.insert("startDate".to_owned(), serde_json::to_value(value)?);
        }

        body.insert("state".to_owned(), serde_json::to_value(&self.state)?);

        if let Some(value) = &self.synced {
            body.insert("synced".to_owned(), serde_json::to_value(value)?);
        }

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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct GetIssuesForSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    expand: Option<Vec<String>>,
    jql: Option<String>,
    max_results: Option<i64>,
    validate_query: Option<bool>,
    fields: Option<Vec<String>>,
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
    pub fn expand(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.expand = Some(value.into_iter().map(Into::into).collect());

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
    pub fn fields(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.fields = Some(value.into_iter().map(Into::into).collect());

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
            config.query.push(("expand".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
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
            config.query.push(("fields".to_owned(), crate::core::QueryValue::Scalar(value.join(","))));
        }

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        Ok(config)
    }

    /// Every item the request matches, one page fetched at a time.
    ///
    /// The first page uses this request; later pages use the next offset, page number, or cursor. The stream ends
    /// at the last page, at a known total, or when no items or next position remain. Reading it needs `TryStreamExt`,
    /// re-exported as [`crate::futures_util`].
    pub fn stream(self) -> futures_util::stream::BoxStream<'a, crate::core::Result<Issue>> {
        let first = self.start_at.unwrap_or(0);

        crate::core::stream_pages(self, first, |mut request, position| async move {
            request.start_at = Some(position);
            let page = request.send().await?;
            let items = page.issues.unwrap_or_default();
            Ok(crate::core::PageStep::offset(items, page.start_at.unwrap_or(position), false, page.total, None))
        })
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
#[derive(Clone)]
pub struct MoveIssuesToSprintAndRankRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    issue_rank_request: IssueAssignRequest,
}

impl<'a> MoveIssuesToSprintAndRankRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64, issue_rank_request: IssueAssignRequest) -> Self {
        Self { client, sprint_id, issue_rank_request }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}/issue", self.sprint_id),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.issue_rank_request)?));

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
#[derive(Clone)]
pub struct GetPropertiesKeysRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: String,
}

impl<'a> GetPropertiesKeysRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: impl Into<String>) -> Self {
        Self { client, sprint_id: sprint_id.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!("/rest/agile/1.0/sprint/{}/properties", crate::core::encode_path_segment(&self.sprint_id)),
        );

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<PropertyKeys> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Returns the value of the property with a given key from the sprint identified by the provided id. The user who retrieves the property is required to have permissions to view the sprint.
#[derive(Clone)]
pub struct GetPropertyRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: String,
    property_key: String,
}

impl<'a> GetPropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, sprint_id: sprint_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::GET,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                crate::core::encode_path_segment(&self.sprint_id),
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
#[derive(Clone)]
pub struct SetPropertyRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: String,
    property_key: String,
    property_value: std::collections::HashMap<String, serde_json::Value>,
}

impl<'a> SetPropertyRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        sprint_id: impl Into<String>,
        property_key: impl Into<String>,
        property_value: std::collections::HashMap<String, serde_json::Value>,
    ) -> Self {
        Self { client, sprint_id: sprint_id.into(), property_key: property_key.into(), property_value }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                crate::core::encode_path_segment(&self.sprint_id),
                crate::core::encode_path_segment(&self.property_key)
            ),
        );

        config.body = Some(crate::core::Body::Json(serde_json::to_value(&self.property_value)?));

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
#[derive(Clone)]
pub struct DeletePropertyRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: String,
    property_key: String,
}

impl<'a> DeletePropertyRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: impl Into<String>, property_key: impl Into<String>) -> Self {
        Self { client, sprint_id: sprint_id.into(), property_key: property_key.into() }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let config = crate::core::RequestConfig::new(
            crate::core::Method::DELETE,
            format!(
                "/rest/agile/1.0/sprint/{}/properties/{}",
                crate::core::encode_path_segment(&self.sprint_id),
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
#[derive(Clone)]
pub struct SwapSprintRequest<'a> {
    client: &'a crate::core::Client,
    sprint_id: i64,
    sprint_to_swap_with: Option<i64>,
    swap: Option<i64>,
}

impl<'a> SwapSprintRequest<'a> {
    fn new(client: &'a crate::core::Client, sprint_id: i64) -> Self {
        Self { client, sprint_id, sprint_to_swap_with: None, swap: None }
    }

    #[must_use]
    pub fn sprint_to_swap_with(mut self, value: i64) -> Self {
        self.sprint_to_swap_with = Some(value);

        self
    }

    #[must_use]
    pub fn swap(mut self, value: i64) -> Self {
        self.swap = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            format!("/rest/agile/1.0/sprint/{}/swap", self.sprint_id),
        );

        let mut body = serde_json::Map::new();

        if let Some(value) = &self.sprint_to_swap_with {
            body.insert("sprintToSwapWith".to_owned(), serde_json::to_value(value)?);
        }

        if let Some(value) = &self.swap {
            body.insert("swap".to_owned(), serde_json::to_value(value)?);
        }

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
