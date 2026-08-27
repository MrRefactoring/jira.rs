// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectRoleActorsUpdate {
    #[serde(rename = "categorisedActors", default, skip_serializing_if = "Option::is_none")]
    pub categorised_actors: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
