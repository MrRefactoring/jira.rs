// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;

crate::open_enum! {
    /// The type indicating the object that contains the entity properties.
    pub enum UpdateEntityPropertiesValueRequestEntityType {
        IssueProperty => "IssueProperty",
        CommentProperty => "CommentProperty",
        DashboardItemProperty => "DashboardItemProperty",
        IssueTypeProperty => "IssueTypeProperty",
        ProjectProperty => "ProjectProperty",
        UserProperty => "UserProperty",
        WorklogProperty => "WorklogProperty",
        BoardProperty => "BoardProperty",
        SprintProperty => "SprintProperty",
    }
}

/// The AppMigration operations.
pub struct AppMigrationService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> AppMigrationService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Updates the value of a custom field added by Connect apps on one or more issues.
    /// The values of up to 200 custom fields can be updated.
    ///
    /// **[Permissions](#permissions) required:** Only Connect apps can make this request
    pub fn update_issue_fields(
        &self,
        atlassian_transfer_id: impl Into<String>,
        connect_custom_field_values: ConnectCustomFieldValues,
    ) -> UpdateIssueFieldsRequest<'a> {
        UpdateIssueFieldsRequest::new(self.client, atlassian_transfer_id, connect_custom_field_values)
    }

    /// Updates the values of multiple entity properties for an object, up to 50 updates per request. This operation is for use by Connect apps during app migration.
    pub fn update_entity_properties_value(
        &self,
        atlassian_transfer_id: impl Into<String>,
        entity_type: impl Into<UpdateEntityPropertiesValueRequestEntityType>,
        body: impl IntoIterator<Item = EntityPropertyDetails>,
    ) -> UpdateEntityPropertiesValueRequest<'a> {
        UpdateEntityPropertiesValueRequest::new(self.client, atlassian_transfer_id, entity_type, body)
    }

    /// Returns configurations for workflow transition rules migrated from server to cloud and owned by the calling Connect app.
    pub fn workflow_rule_search(
        &self,
        atlassian_transfer_id: impl Into<String>,
        workflow_rules_search: WorkflowRulesSearch,
    ) -> WorkflowRuleSearchRequest<'a> {
        WorkflowRuleSearchRequest::new(self.client, atlassian_transfer_id, workflow_rules_search)
    }
}

/// Updates the value of a custom field added by Connect apps on one or more issues.
/// The values of up to 200 custom fields can be updated.
///
/// **[Permissions](#permissions) required:** Only Connect apps can make this request
pub struct UpdateIssueFieldsRequest<'a> {
    client: &'a crate::core::Client,
    atlassian_transfer_id: String,
    connect_custom_field_values: ConnectCustomFieldValues,
}

impl<'a> UpdateIssueFieldsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        atlassian_transfer_id: impl Into<String>,
        connect_custom_field_values: ConnectCustomFieldValues,
    ) -> Self {
        Self { client, atlassian_transfer_id: atlassian_transfer_id.into(), connect_custom_field_values }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/atlassian-connect/1/migration/field".to_owned(),
        );

        config.headers.push(("Atlassian-Transfer-Id".to_owned(), self.atlassian_transfer_id.clone()));

        let body = match serde_json::to_value(&self.connect_custom_field_values)? {
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

/// Updates the values of multiple entity properties for an object, up to 50 updates per request. This operation is for use by Connect apps during app migration.
pub struct UpdateEntityPropertiesValueRequest<'a> {
    client: &'a crate::core::Client,
    atlassian_transfer_id: String,
    entity_type: UpdateEntityPropertiesValueRequestEntityType,
    body: Vec<EntityPropertyDetails>,
}

impl<'a> UpdateEntityPropertiesValueRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        atlassian_transfer_id: impl Into<String>,
        entity_type: impl Into<UpdateEntityPropertiesValueRequestEntityType>,
        body: impl IntoIterator<Item = EntityPropertyDetails>,
    ) -> Self {
        Self {
            client,
            atlassian_transfer_id: atlassian_transfer_id.into(),
            entity_type: entity_type.into(),
            body: body.into_iter().collect(),
        }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            format!("/rest/atlassian-connect/1/migration/properties/{}", self.entity_type),
        );

        config.headers.push(("Atlassian-Transfer-Id".to_owned(), self.atlassian_transfer_id.clone()));

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

/// Returns configurations for workflow transition rules migrated from server to cloud and owned by the calling Connect app.
pub struct WorkflowRuleSearchRequest<'a> {
    client: &'a crate::core::Client,
    atlassian_transfer_id: String,
    workflow_rules_search: WorkflowRulesSearch,
}

impl<'a> WorkflowRuleSearchRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        atlassian_transfer_id: impl Into<String>,
        workflow_rules_search: WorkflowRulesSearch,
    ) -> Self {
        Self { client, atlassian_transfer_id: atlassian_transfer_id.into(), workflow_rules_search }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::POST,
            "/rest/atlassian-connect/1/migration/workflow/rule/search".to_owned(),
        );

        config.headers.push(("Atlassian-Transfer-Id".to_owned(), self.atlassian_transfer_id.clone()));

        let body = match serde_json::to_value(&self.workflow_rules_search)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowRulesSearchDetails> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
