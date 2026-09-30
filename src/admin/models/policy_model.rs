// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Type of this object
    pub enum PolicyModelType {
        Policy => "policy",
    }
}

crate::open_enum! {
    /// Type of this Policy
    pub enum PolicyModelAttributesType {
        IpAllowlist => "ip-allowlist",
        DataResidency => "data-residency",
        DataSecurity => "data-security",
        AdminNotificationSettings => "admin-notification-settings",
        GenerativeAi => "generative-ai",
        Hipaa => "hipaa",
        UgcDataUsePreferences => "ugc-data-use-preferences",
        UserJoinSettingsDefault => "user-join-settings-default",
    }
}

crate::open_enum! {
    /// Status of this Policy
    pub enum PolicyModelAttributesStatus {
        Enabled => "enabled",
        Disabled => "disabled",
    }
}

/// Rule of the Policy
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum PolicyModelAttributesRule {
    AllowIfContainedRule(AllowIfContainedRule),
    Variant1(Vec<serde_json::Value>),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

/// Attributes of this object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelAttributes {
    /// Type of this Policy
    pub r#type: PolicyModelAttributesType,
    /// Name of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Status of this Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PolicyModelAttributesStatus>,
    /// Rule of the Policy
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<PolicyModelAttributesRule>,
    /// list of resources Policy is associated with
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "queryData", default, skip_serializing_if = "Option::is_none")]
    pub query_data: Option<serde_json::Value>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModel {
    /// Unique identifier of the Policy
    pub id: String,
    /// Type of this object
    pub r#type: PolicyModelType,
    /// Attributes of this object
    pub attributes: PolicyModelAttributes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<serde_json::Value>,
}
