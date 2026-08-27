// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeHistory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<UserJson>,
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    #[serde(rename = "historyMetadata", default, skip_serializing_if = "Option::is_none")]
    pub history_metadata: Option<HistoryMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ChangeItem>>,
}
