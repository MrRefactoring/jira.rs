// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexIntegrityOut {
    #[serde(rename = "objectJiraIssueIndexOk", default, skip_serializing_if = "Option::is_none")]
    pub object_jira_issue_index_ok: Option<bool>,
    #[serde(rename = "objectSchemaIndexOk", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_index_ok: Option<bool>,
    #[serde(rename = "objectTypeAttributeIndexOk", default, skip_serializing_if = "Option::is_none")]
    pub object_type_attribute_index_ok: Option<bool>,
    #[serde(rename = "objectTypeIndexOk", default, skip_serializing_if = "Option::is_none")]
    pub object_type_index_ok: Option<bool>,
    #[serde(rename = "objectIndexOk", default, skip_serializing_if = "Option::is_none")]
    pub object_index_ok: Option<bool>,
    #[serde(rename = "reindexNeeded", default, skip_serializing_if = "Option::is_none")]
    pub reindex_needed: Option<bool>,
}
