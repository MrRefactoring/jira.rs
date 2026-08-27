// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details a link group, which defines issue operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LinkGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Box<LinkGroup>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<SimpleLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SimpleLink>>,
    #[serde(rename = "styleClass", default, skip_serializing_if = "Option::is_none")]
    pub style_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
