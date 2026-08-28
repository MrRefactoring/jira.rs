// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "emptyAdf", default, skip_serializing_if = "Option::is_none")]
    pub empty_adf: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalised: Option<bool>,
    #[serde(rename = "valueSet", default, skip_serializing_if = "Option::is_none")]
    pub value_set: Option<bool>,
}
