// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntitlementModelV2Attributes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "planKey", default, skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[serde(rename = "billingSourceSystem", default, skip_serializing_if = "Option::is_none")]
    pub billing_source_system: Option<String>,
    #[serde(rename = "ccpEntitlementId", default, skip_serializing_if = "Option::is_none")]
    pub ccp_entitlement_id: Option<String>,
    #[serde(rename = "ccpTransactionAccountId", default, skip_serializing_if = "Option::is_none")]
    pub ccp_transaction_account_id: Option<String>,
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "softCapacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub soft_capacity_limit: Option<f64>,
    #[serde(rename = "usageIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub usage_identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntitlementModelV2Links {
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntitlementModelV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<EntitlementModelV2Attributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<EntitlementModelV2Links>,
}
