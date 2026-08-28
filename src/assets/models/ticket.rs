// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ticket {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "globalId")]
    pub global_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub id: String,
    pub reporter: String,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub created: String,
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    #[cfg(not(feature = "chrono"))]
    #[serde(deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub updated: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
    pub r#type: TicketType,
    pub priority: TicketPriority,
}
