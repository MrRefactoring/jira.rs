// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Whether the approval configuration is active.
    pub enum ApprovalConfigurationActive {
        True => "true",
        False => "false",
    }
}

crate::open_enum! {
    /// How the required approval count is calculated. It may be configured to require a specific number of approvals, or approval by a percentage of approvers. If the approvers source field is Approver groups, you can configure how many approvals per group are required for the request to be approved. The number will be the same across all groups.
    pub enum ApprovalConfigurationConditionType {
        Number => "number",
        Percent => "percent",
        NumberPerPrincipal => "numberPerPrincipal",
    }
}

crate::open_enum! {
    /// A list of roles that should be excluded as possible approvers.
    pub enum ApprovalConfigurationExclude {
        Assignee => "assignee",
        Reporter => "reporter",
    }
}

/// The approval configuration of a status within a workflow. Applies only to Jira Service Management approvals.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprovalConfiguration {
    /// Whether the approval configuration is active.
    pub active: ApprovalConfigurationActive,
    /// How the required approval count is calculated. It may be configured to require a specific number of approvals, or approval by a percentage of approvers. If the approvers source field is Approver groups, you can configure how many approvals per group are required for the request to be approved. The number will be the same across all groups.
    #[serde(rename = "conditionType")]
    pub condition_type: ApprovalConfigurationConditionType,
    /// The number or percentage of approvals required for a request to be approved. If `conditionType` is `number`, the value must be 20 or less. If `conditionType` is `percent`, the value must be 100 or less.
    #[serde(rename = "conditionValue")]
    pub condition_value: String,
    /// A list of roles that should be excluded as possible approvers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<Option<ApprovalConfigurationExclude>>>,
    /// The custom field ID of the "Approvers" or "Approver Groups" field.
    #[serde(rename = "fieldId")]
    pub field_id: String,
    /// The custom field ID of the field used to pre-populate the Approver field. Only supports the "Affected Services" field.
    #[serde(rename = "prePopulatedFieldId", default, skip_serializing_if = "Option::is_none")]
    pub pre_populated_field_id: Option<String>,
    /// The numeric ID of the transition to be executed if the request is approved.
    #[serde(rename = "transitionApproved")]
    pub transition_approved: String,
    /// The numeric ID of the transition to be executed if the request is declined.
    #[serde(rename = "transitionRejected")]
    pub transition_rejected: String,
}
