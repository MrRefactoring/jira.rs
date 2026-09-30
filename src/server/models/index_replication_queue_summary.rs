// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IndexReplicationQueueSummary {
    #[serde(rename = "lastConsumedOperation", default, skip_serializing_if = "Option::is_none")]
    pub last_consumed_operation: Option<IndexReplicationQueueEntry>,
    #[serde(rename = "lastOperationInQueue", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_in_queue: Option<IndexReplicationQueueEntry>,
    #[serde(rename = "queueSize", default, skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<i64>,
}
