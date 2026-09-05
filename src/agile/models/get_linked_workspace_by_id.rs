// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The Security Workspace information stored for the given ID.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetLinkedWorkspaceById {
    /// The Security Workspace ID
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    /// Latest date and time that the Security Workspace was updated in Jira.
    #[cfg(feature = "chrono")]
    #[serde(
        rename = "updatedAt",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Latest date and time that the Security Workspace was updated in Jira.
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "updatedAt", deserialize_with = "crate::core::deserialize_required_timestamp")]
    pub updated_at: String,
}
