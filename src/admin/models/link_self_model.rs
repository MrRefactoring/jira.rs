// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LinkSelfModel {
    /// URL to fetch this resource
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
