// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Configuration of features for one or more boards. Replaces the deprecated features field on BoardPayload
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardFeaturesPayload {
    /// A map of board PCRIs to the list of features to enable on each board.
    #[serde(rename = "boardFeatures", default, skip_serializing_if = "Option::is_none")]
    pub board_features: Option<std::collections::HashMap<String, serde_json::Value>>,
}
