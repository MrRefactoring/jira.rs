// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The view type of the context.
    /// Supported values:
    ///
    ///  *  `GIC` \- Jira global issue create
    ///  *  `IssueView` \- Jira issue view
    ///  *  `IssueTransition` \- Jira issue transition
    ///  *  `JSMRequestCreate` \- Jira Service Management request create portal view
    ///  *  `GICAgentView` \- Agent view variant of Jira global issue create
    ///  *  `IssueViewAgentView` \- Agent view variant of Jira issue view
    ///  *  `IssueTransitionAgentView` \- Agent view variant of Jira issue transition
    ///
    /// For Jira and Agent view types (`GIC`, `IssueView`, `IssueTransition`, `GICAgentView`, `IssueViewAgentView`, `IssueTransitionAgentView`), null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each Jira or Agent context can have a maximum of one wildcard.
    ///
    /// Agent view contexts use `projectId` and `issueTypeId` like Jira contexts, and may optionally also set `requestTypeId`. Agent view contexts must not set `portalId`.
    ///
    /// Wildcards are not applicable for JSM contexts.
    pub enum UiModificationContextDetailsViewType {
        Gic => "GIC",
        IssueView => "IssueView",
        IssueTransition => "IssueTransition",
        JSMRequestCreate => "JSMRequestCreate",
        GICAgentView => "GICAgentView",
        IssueViewAgentView => "IssueViewAgentView",
        IssueTransitionAgentView => "IssueTransitionAgentView",
    }
}

/// The details of a UI modification's context, which define where to activate the UI modification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiModificationContextDetails {
    /// The ID of the UI modification context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether a context is available. For example, when a project is deleted the context becomes unavailable.
    #[serde(rename = "isAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    /// The issue type ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all issue types. Each UI modification context can have a maximum of one wildcard.
    #[serde(rename = "issueTypeId", default, skip_serializing_if = "Option::is_none")]
    pub issue_type_id: Option<String>,
    /// The portal ID of the context. Only required for Jira Service Management request create portal view (`JSMRequestCreate`).
    #[serde(rename = "portalId", default, skip_serializing_if = "Option::is_none")]
    pub portal_id: Option<String>,
    /// The project ID of the context. Null is treated as a wildcard, meaning the UI modification will be applied to all projects. Each UI modification context can have a maximum of one wildcard.
    #[serde(rename = "projectId", default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The request type ID of the context. Required for Jira Service Management request create portal view (`JSMRequestCreate`). Optional for Agent view types (`GICAgentView`, `IssueViewAgentView`, `IssueTransitionAgentView`): when set on an agent view context, the UI modification applies only to issues with that request type. Omitting `requestTypeId` does not create a wildcard — it means the context is not scoped to any specific request type.
    #[serde(rename = "requestTypeId", default, skip_serializing_if = "Option::is_none")]
    pub request_type_id: Option<String>,
    /// The view type of the context.
    /// Supported values:
    ///
    ///  *  `GIC` \- Jira global issue create
    ///  *  `IssueView` \- Jira issue view
    ///  *  `IssueTransition` \- Jira issue transition
    ///  *  `JSMRequestCreate` \- Jira Service Management request create portal view
    ///  *  `GICAgentView` \- Agent view variant of Jira global issue create
    ///  *  `IssueViewAgentView` \- Agent view variant of Jira issue view
    ///  *  `IssueTransitionAgentView` \- Agent view variant of Jira issue transition
    ///
    /// For Jira and Agent view types (`GIC`, `IssueView`, `IssueTransition`, `GICAgentView`, `IssueViewAgentView`, `IssueTransitionAgentView`), null is treated as a wildcard, meaning the UI modification will be applied to all view types. Each Jira or Agent context can have a maximum of one wildcard.
    ///
    /// Agent view contexts use `projectId` and `issueTypeId` like Jira contexts, and may optionally also set `requestTypeId`. Agent view contexts must not set `portalId`.
    ///
    /// Wildcards are not applicable for JSM contexts.
    #[serde(rename = "viewType", default, skip_serializing_if = "Option::is_none")]
    pub view_type: Option<UiModificationContextDetailsViewType>,
}
