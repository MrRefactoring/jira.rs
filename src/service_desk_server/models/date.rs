// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Date {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso8601: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jira: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly: Option<String>,
    #[serde(rename = "epochMillis", default, skip_serializing_if = "Option::is_none")]
    pub epoch_millis: Option<i64>,
}
