// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexSummary {
    #[serde(rename = "externalPlatformIndexReplay", default, skip_serializing_if = "Option::is_none")]
    pub external_platform_index_replay: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "issueIndex", default, skip_serializing_if = "Option::is_none")]
    pub issue_index: Option<IssueIndexSummary>,
    #[serde(rename = "nodeId", default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "replicationQueues", default, skip_serializing_if = "Option::is_none")]
    pub replication_queues: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(
        rename = "reportTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub report_time: Option<String>,
}
