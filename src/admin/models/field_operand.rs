// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Returns workspaces where a specified event field has one of the specified values.Absent of values makes this operator no-op.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldOperandField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Returns workspaces where a specified event field has one of the specified values. Absent of values makes this operator no-op.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldOperand {
    /// Returns workspaces where a specified event field has one of the specified values.Absent of values makes this operator no-op.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldOperandField>,
}
