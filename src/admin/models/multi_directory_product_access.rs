// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Last active timestamps for the user by product.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryProductAccess {
    /// The product key (e.g. `jira`, `confluence`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// the unique ID of the Product instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ISO-8601 timestamp of the user's last activity in the given product and site.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "lastActiveTimestamp",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub last_active_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// ISO-8601 timestamp of the user's last activity in the given product and site.
    #[cfg(not(feature = "chrono"))]
    #[serde(
        rename = "lastActiveTimestamp",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub last_active_timestamp: Option<String>,
}
