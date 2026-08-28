// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// The linked item.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The summary details of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The title of the item.
    pub title: String,
    /// The URL of the item.
    pub url: String,
    /// Keys the specification does not describe, kept rather than dropped.
    #[serde(flatten)]
    pub additional: std::collections::HashMap<String, serde_json::Value>,
}
