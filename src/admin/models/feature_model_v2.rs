// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2AttributesParent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2AttributesPortal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2AttributesSelf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2Attributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "allInclusive", default, skip_serializing_if = "Option::is_none")]
    pub all_inclusive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "entitledSandbox", default, skip_serializing_if = "Option::is_none")]
    pub entitled_sandbox: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<f64>,
    #[serde(rename = "isDataResidencyAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_data_residency_allowed: Option<bool>,
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "isEntitled", default, skip_serializing_if = "Option::is_none")]
    pub is_entitled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<FeatureModelV2AttributesParent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portal: Option<FeatureModelV2AttributesPortal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub realms: Option<Vec<String>>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<FeatureModelV2AttributesSelf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "userAccessManagementState", default, skip_serializing_if = "Option::is_none")]
    pub user_access_management_state: Option<String>,
    #[serde(rename = "workspaceActive", default, skip_serializing_if = "Option::is_none")]
    pub workspace_active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2Links {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FeatureModelV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<FeatureModelV2Attributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<FeatureModelV2Links>,
}
