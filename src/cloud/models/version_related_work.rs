// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Associated related work to a version
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionRelatedWork {
    /// The category of the related work
    pub category: String,
    /// The ID of the issue associated with the related work (if there is one). Cannot be updated via the Rest API.
    #[serde(rename = "issueId", default, skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<i64>,
    /// The id of the related work. For the native release note related work item, this will be null, and Rest API does not support updating it.
    #[serde(rename = "relatedWorkId", default, skip_serializing_if = "Option::is_none")]
    pub related_work_id: Option<String>,
    /// The title of the related work
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL of the related work. Will be null for the native release note related work item, but is otherwise required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
