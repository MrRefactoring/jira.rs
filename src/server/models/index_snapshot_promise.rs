// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndexSnapshotPromise {
    #[serde(rename = "futureAbsolutePath", default, skip_serializing_if = "Option::is_none")]
    pub future_absolute_path: Option<String>,
}
