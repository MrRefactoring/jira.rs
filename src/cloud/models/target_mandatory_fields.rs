// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Field mapping for mandatory fields in target
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetMandatoryFields {
    /// Contains the value of mandatory fields
    pub fields: std::collections::HashMap<String, serde_json::Value>,
}
