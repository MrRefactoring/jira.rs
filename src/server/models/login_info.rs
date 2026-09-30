// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LoginInfo {
    #[serde(rename = "failedLoginCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_login_count: Option<i64>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastFailedLoginTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_failed_login_time: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastFailedLoginTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_failed_login_time: Option<String>,
    #[serde(rename = "loginCount", default, skip_serializing_if = "Option::is_none")]
    pub login_count: Option<i64>,
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "previousLoginTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub previous_login_time: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "previousLoginTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub previous_login_time: Option<String>,
}
