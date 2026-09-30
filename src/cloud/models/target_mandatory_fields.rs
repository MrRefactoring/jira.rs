// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Field mapping for mandatory fields in target
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TargetMandatoryFields {
    /// Contains the value of mandatory fields
    pub fields: std::collections::HashMap<String, MandatoryFieldValues>,
}
