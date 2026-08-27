// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A project category.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectCategory {
    /// The description of the project category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the project category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the project category. Required on create, optional on update.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the project category.
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
