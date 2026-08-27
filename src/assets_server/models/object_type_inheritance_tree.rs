// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObjectTypeInheritanceTree {
    #[serde(rename = "parentObjectTypeIdsInclSelf", default, skip_serializing_if = "Option::is_none")]
    pub parent_object_type_ids_incl_self: Option<std::collections::HashMap<String, serde_json::Value>>,
}
