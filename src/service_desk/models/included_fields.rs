// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IncludedFields {
    #[serde(rename = "actuallyIncluded", default, skip_serializing_if = "Option::is_none")]
    pub actually_included: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excluded: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<String>>,
}
