// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EventObjectModelLinks {
    LinkSelfAltModel(LinkSelfAltModel),
    LinkAltModel(LinkAltModel),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EventObjectModel {
    /// Unique identifier of the event object
    pub id: String,
    /// Type name of this object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Attributes of this object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<EventObjectModelLinks>,
}
