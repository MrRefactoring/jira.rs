// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The Assets Group type
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Group {
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    pub name: String,
}
