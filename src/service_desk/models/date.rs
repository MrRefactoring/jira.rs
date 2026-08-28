// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Date {
    /// Date as the number of milliseconds that have elapsed since 00:00:00 Coordinated Universal Time (UTC), 1 January 1970.
    #[serde(rename = "epochMillis", default, skip_serializing_if = "Option::is_none")]
    pub epoch_millis: Option<i64>,
    /// Date in a user-friendly text format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub friendly: Option<String>,
    /// Date in ISO8601 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iso8601: Option<String>,
    /// Date in the format used in the Jira REST APIs, which is ISO8601 format but extended with milliseconds. For example, 2016-09-28T23:08:32.097+1000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jira: Option<String>,
}
