// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The item section type
    pub enum IssueLayoutItemPayloadSectionType {
        Content => "content",
        PrimaryContext => "primaryContext",
        SecondaryContext => "secondaryContext",
    }
}

crate::open_enum! {
    /// The item type. Currently only support FIELD
    pub enum IssueLayoutItemPayloadType {
        Field => "FIELD",
    }
}

/// Defines the payload to configure the issue layout item for a project.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueLayoutItemPayload {
    #[serde(rename = "itemKey", default, skip_serializing_if = "Option::is_none")]
    pub item_key: Option<ProjectCreateResourceIdentifier>,
    /// Additional properties for this item. This field is only used when the type is FIELD.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The item section type
    #[serde(rename = "sectionType", default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<IssueLayoutItemPayloadSectionType>,
    /// The item type. Currently only support FIELD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IssueLayoutItemPayloadType>,
}
