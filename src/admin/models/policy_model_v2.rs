// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelV2AttributesParent {
    #[serde(rename = "availableForCustomDomain", default, skip_serializing_if = "Option::is_none")]
    pub available_for_custom_domain: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelV2AttributesPortal {
    #[serde(rename = "availableForCustomDomain", default, skip_serializing_if = "Option::is_none")]
    pub available_for_custom_domain: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelV2Attributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "availableTargets", default, skip_serializing_if = "Option::is_none")]
    pub available_targets: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<PolicyModelV2AttributesParent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub portal: Option<PolicyModelV2AttributesPortal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelV2Links {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PolicyModelV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<PolicyModelV2Attributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<PolicyModelV2Links>,
}
