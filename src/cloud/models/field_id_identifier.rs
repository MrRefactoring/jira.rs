// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldIdIdentifier {
    #[serde(flatten)]
    pub field_identifier_object: FieldIdentifierObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
