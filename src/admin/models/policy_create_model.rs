// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Type of this object
    pub enum PolicyCreateModelType {
        Policy => "policy",
    }
}

crate::open_enum! {
    /// Type of this Policy
    pub enum PolicyCreateModelAttributesType {
        IpAllowlist => "ip-allowlist",
        DataResidency => "data-residency",
    }
}

crate::open_enum! {
    /// Status of this Policy
    pub enum PolicyCreateModelAttributesStatus {
        Enabled => "enabled",
        Disabled => "disabled",
    }
}

/// Rule of the Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum PolicyCreateModelAttributesRule {
    AllowIfContainedRule(AllowIfContainedRule),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Attributes of this object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyCreateModelAttributes {
    /// Type of this Policy
    pub r#type: PolicyCreateModelAttributesType,
    /// Name of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PolicyCreateModelAttributesStatus>,
    /// Rule of the Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<PolicyCreateModelAttributesRule>,
    /// list of resources Policy is associated with
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceInput>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyCreateModel {
    /// Type of this object
    pub r#type: PolicyCreateModelType,
    /// Attributes of this object
    pub attributes: PolicyCreateModelAttributes,
}
