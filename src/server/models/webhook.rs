// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(rename = "updatedDate", default, skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    #[serde(rename = "sslVerificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub ssl_verification_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<WebhookStatistics>,
}
