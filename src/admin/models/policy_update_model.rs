// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Type of this object
    pub enum PolicyUpdateModelType {
        Policy => "policy",
    }
}

crate::open_enum! {
    /// Type of this Policy
    pub enum PolicyUpdateModelAttributesType {
        IpAllowlist => "ip-allowlist",
        DataResidency => "data-residency",
    }
}

crate::open_enum! {
    /// Status of this Policy
    pub enum PolicyUpdateModelAttributesStatus {
        Enabled => "enabled",
        Disabled => "disabled",
    }
}

/// Rule of the Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum PolicyUpdateModelAttributesRule {
    AllowIfContainedRule(AllowIfContainedRule),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Attributes of this object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyUpdateModelAttributes {
    /// Type of this Policy
    pub r#type: PolicyUpdateModelAttributesType,
    /// Name of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PolicyUpdateModelAttributesStatus>,
    /// Rule of the Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<PolicyUpdateModelAttributesRule>,
    /// list of resources Policy is associated with
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceInput>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyUpdateModel {
    /// Unique identifier of the Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Type of this object
    pub r#type: PolicyUpdateModelType,
    /// Attributes of this object
    pub attributes: PolicyUpdateModelAttributes,
}
