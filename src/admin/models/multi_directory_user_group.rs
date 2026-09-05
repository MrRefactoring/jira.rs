// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// A group the user is a member of within the requested directory.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserGroup {
    /// the unique ID of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The display name of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
