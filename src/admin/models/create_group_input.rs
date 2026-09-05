// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateGroupInput {
    /// The name of the group.
    pub name: String,
    /// The description of the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
