// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Comment {
    /// List of items that can be expanded in the response by specifying the expand query parameter.
    #[serde(rename = "_expands", default, skip_serializing_if = "Option::is_none")]
    pub expands: Option<Vec<String>>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<SelfLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<PagedAttachment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    /// Content of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<Date>,
    /// ID of the comment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates whether the comment is public (true) or private/internal (false).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "renderedBody", default, skip_serializing_if = "Option::is_none")]
    pub rendered_body: Option<RenderedValue>,
}
