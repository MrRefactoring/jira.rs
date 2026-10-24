// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QueueCountSettings {
    #[serde(rename = "includeQueueCount")]
    pub include_queue_count: bool,
    #[serde(rename = "useCachedQueueCount")]
    pub use_cached_queue_count: bool,
}
