// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServiceManagementNavigationInfo {
    #[serde(rename = "queueCategory", default, skip_serializing_if = "Option::is_none")]
    pub queue_category: Option<String>,
    #[serde(rename = "queueId", default, skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i64>,
    #[serde(rename = "queueName", default, skip_serializing_if = "Option::is_none")]
    pub queue_name: Option<String>,
}
