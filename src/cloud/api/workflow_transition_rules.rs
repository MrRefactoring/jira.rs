// @generated. Do not edit: change the generator or the specification.

use super::super::models::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GetWorkflowTransitionRuleConfigurationsRequestTypes {
        Postfunction => "postfunction",
        Condition => "condition",
        Validator => "validator",
    }
}

crate::open_enum! {
    pub enum GetWorkflowTransitionRuleConfigurationsRequestExpandValue {
        Transition => "transition",
    }
}

/// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts `transition`, which, for each rule, returns information about the transition the rule is assigned to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GetWorkflowTransitionRuleConfigurationsRequestExpand {
    One(GetWorkflowTransitionRuleConfigurationsRequestExpandValue),
    Many(Vec<GetWorkflowTransitionRuleConfigurationsRequestExpandValue>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// The WorkflowTransitionRules operations.
pub struct WorkflowTransitionRulesService<'a> {
    client: &'a crate::core::Client,
}

impl<'a> WorkflowTransitionRulesService<'a> {
    pub(crate) fn new(client: &'a crate::core::Client) -> Self {
        Self { client }
    }

    /// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of workflows with transition rules. The workflows can be filtered to return only those containing workflow transition rules:
    ///
    ///  *  of one or more transition rule types, such as [workflow post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/).
    ///  *  matching one or more transition rule keys.
    ///
    /// Only workflows containing transition rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app are returned.
    ///
    /// Due to server-side optimizations, workflows with an empty list of rules may be returned; these workflows can be ignored.
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.
    pub fn get_workflow_transition_rule_configurations(
        &self,
        types: impl IntoIterator<Item = impl Into<GetWorkflowTransitionRuleConfigurationsRequestTypes>>,
    ) -> GetWorkflowTransitionRuleConfigurationsRequest<'a> {
        GetWorkflowTransitionRuleConfigurationsRequest::new(self.client, types)
    }

    /// Updates configuration of workflow transition rules. The following rule types are supported:
    ///
    ///  *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)
    ///  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)
    ///  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)
    ///
    /// Only rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app can be updated.
    ///
    /// To assist with app migration, this operation can be used to:
    ///
    ///  *  Disable a rule.
    ///  *  Add a `tag`. Use this to filter rules in the [Get workflow transition rule configurations](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-workflow-transition-rules/#api-rest-api-3-workflow-rule-config-get).
    ///
    /// Rules are enabled if the `disabled` parameter is not provided.
    ///
    /// **Note:** The `draft` parameter in the request body WorkflowId is deprecated and will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.
    pub fn update_workflow_transition_rule_configurations(
        &self,
        workflow_transition_rules_update: WorkflowTransitionRulesUpdate,
    ) -> UpdateWorkflowTransitionRuleConfigurationsRequest<'a> {
        UpdateWorkflowTransitionRuleConfigurationsRequest::new(self.client, workflow_transition_rules_update)
    }

    /// Deletes workflow transition rules from one or more workflows. These rule types are supported:
    ///
    ///  *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)
    ///  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)
    ///  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)
    ///
    /// Only rules created by the calling Connect app can be deleted.
    ///
    /// **Note:** The `draft` parameter in the request body WorkflowId is deprecated and will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
    ///
    /// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect apps can use this operation.
    pub fn delete_workflow_transition_rule_configurations(
        &self,
        workflows_with_transition_rules_details: WorkflowsWithTransitionRulesDetails,
    ) -> DeleteWorkflowTransitionRuleConfigurationsRequest<'a> {
        DeleteWorkflowTransitionRuleConfigurationsRequest::new(self.client, workflows_with_transition_rules_details)
    }
}

/// Returns a [paginated](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#pagination) list of workflows with transition rules. The workflows can be filtered to return only those containing workflow transition rules:
///
///  *  of one or more transition rule types, such as [workflow post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/).
///  *  matching one or more transition rule keys.
///
/// Only workflows containing transition rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app are returned.
///
/// Due to server-side optimizations, workflows with an empty list of rules may be returned; these workflows can be ignored.
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.
#[derive(Clone)]
pub struct GetWorkflowTransitionRuleConfigurationsRequest<'a> {
    client: &'a crate::core::Client,
    start_at: Option<i64>,
    max_results: Option<i64>,
    types: Vec<GetWorkflowTransitionRuleConfigurationsRequestTypes>,
    keys: Option<Vec<String>>,
    workflow_names: Option<Vec<String>>,
    with_tags: Option<Vec<String>>,
    draft: Option<bool>,
    expand: Option<GetWorkflowTransitionRuleConfigurationsRequestExpand>,
}

impl<'a> GetWorkflowTransitionRuleConfigurationsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        types: impl IntoIterator<Item = impl Into<GetWorkflowTransitionRuleConfigurationsRequestTypes>>,
    ) -> Self {
        Self {
            client,
            types: types.into_iter().map(Into::into).collect(),
            start_at: None,
            max_results: None,
            keys: None,
            workflow_names: None,
            with_tags: None,
            draft: None,
            expand: None,
        }
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

    /// The transition rule class keys, as defined in the Connect or the Forge app descriptor, of the transition rules to return.
    #[must_use]
    pub fn keys(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keys = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of workflow names to filter by.
    #[must_use]
    pub fn workflow_names(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.workflow_names = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// The list of `tags` to filter by.
    #[must_use]
    pub fn with_tags(mut self, value: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.with_tags = Some(value.into_iter().map(Into::into).collect());

        self
    }

    /// **Deprecated:** Whether draft or published workflows are returned. If not provided, both workflow types are returned. The 'draft' parameter will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
    #[deprecated(note = "**Deprecated:** Whether draft or published workflows are returned.")]
    #[must_use]
    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);

        self
    }

    /// Use [expand](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#expansion) to include additional information in the response. This parameter accepts `transition`, which, for each rule, returns information about the transition the rule is assigned to.
    #[must_use]
    pub fn expand(mut self, value: GetWorkflowTransitionRuleConfigurationsRequestExpand) -> Self {
        self.expand = Some(value);

        self
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::GET, "/rest/api/3/workflow/rule/config".to_owned());

        if let Some(value) = &self.start_at {
            config.query.push(("startAt".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.max_results {
            config.query.push(("maxResults".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        config.query.push(("types".to_owned(), crate::core::QueryValue::from_serializable(&self.types)?));

        if let Some(value) = &self.keys {
            config.query.push(("keys".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.workflow_names {
            config.query.push(("workflowNames".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.with_tags {
            config.query.push(("withTags".to_owned(), crate::core::QueryValue::List(value.clone())));
        }

        if let Some(value) = &self.draft {
            config.query.push(("draft".to_owned(), crate::core::QueryValue::Scalar(value.to_string())));
        }

        if let Some(value) = &self.expand {
            config.query.push(("expand".to_owned(), crate::core::QueryValue::from_serializable(value)?));
        }

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<Page<WorkflowTransitionRules>> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Updates configuration of workflow transition rules. The following rule types are supported:
///
///  *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)
///  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)
///  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)
///
/// Only rules created by the calling [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) app can be updated.
///
/// To assist with app migration, this operation can be used to:
///
///  *  Disable a rule.
///  *  Add a `tag`. Use this to filter rules in the [Get workflow transition rule configurations](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-workflow-transition-rules/#api-rest-api-3-workflow-rule-config-get).
///
/// Rules are enabled if the `disabled` parameter is not provided.
///
/// **Note:** The `draft` parameter in the request body WorkflowId is deprecated and will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only [Connect](https://developer.atlassian.com/cloud/jira/platform/index/#connect-apps) or [Forge](https://developer.atlassian.com/cloud/jira/platform/index/#forge-apps) apps can use this operation.
#[derive(Clone)]
pub struct UpdateWorkflowTransitionRuleConfigurationsRequest<'a> {
    client: &'a crate::core::Client,
    workflow_transition_rules_update: WorkflowTransitionRulesUpdate,
}

impl<'a> UpdateWorkflowTransitionRuleConfigurationsRequest<'a> {
    fn new(client: &'a crate::core::Client, workflow_transition_rules_update: WorkflowTransitionRulesUpdate) -> Self {
        Self { client, workflow_transition_rules_update }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config =
            crate::core::RequestConfig::new(crate::core::Method::PUT, "/rest/api/3/workflow/rule/config".to_owned());

        let body = match serde_json::to_value(&self.workflow_transition_rules_update)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowTransitionRulesUpdateErrors> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}

/// Deletes workflow transition rules from one or more workflows. These rule types are supported:
///
///  *  [post functions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-post-function/)
///  *  [conditions](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-condition/)
///  *  [validators](https://developer.atlassian.com/cloud/jira/platform/modules/workflow-validator/)
///
/// Only rules created by the calling Connect app can be deleted.
///
/// **Note:** The `draft` parameter in the request body WorkflowId is deprecated and will be removed from this API on [November 2, 2026](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-3147).
///
/// **[Permissions](https://developer.atlassian.com/cloud/jira/platform/rest/v3/intro#permissions) required:** Only Connect apps can use this operation.
#[derive(Clone)]
pub struct DeleteWorkflowTransitionRuleConfigurationsRequest<'a> {
    client: &'a crate::core::Client,
    workflows_with_transition_rules_details: WorkflowsWithTransitionRulesDetails,
}

impl<'a> DeleteWorkflowTransitionRuleConfigurationsRequest<'a> {
    fn new(
        client: &'a crate::core::Client,
        workflows_with_transition_rules_details: WorkflowsWithTransitionRulesDetails,
    ) -> Self {
        Self { client, workflows_with_transition_rules_details }
    }

    /// The request as the transport will send it.
    pub fn config(&self) -> crate::core::Result<crate::core::RequestConfig> {
        let mut config = crate::core::RequestConfig::new(
            crate::core::Method::PUT,
            "/rest/api/3/workflow/rule/config/delete".to_owned(),
        );

        let body = match serde_json::to_value(&self.workflows_with_transition_rules_details)? {
            serde_json::Value::Object(object) => object,
            _ => serde_json::Map::new(),
        };

        config.body = Some(crate::core::Body::Json(serde_json::Value::Object(body)));

        Ok(config)
    }

    /// Sends the request.
    pub async fn send(self) -> crate::core::Result<WorkflowTransitionRulesUpdateErrors> {
        self.client.send(&self.config()?).await
    }

    /// Sends the request and hands back the body unmodelled.
    pub async fn send_raw(self) -> crate::core::Result<serde_json::Value> {
        self.client.send_raw(&self.config()?).await
    }
}
