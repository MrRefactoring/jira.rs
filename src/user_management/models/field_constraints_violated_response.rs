// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConstraintsViolatedResponseContextFieldViolationsViolations {
    /// The key for a constraint that the submitted value has violated.
    /// See documentation for any submittable model for a set of constraint keys and definitions.
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConstraintsViolatedResponseContextFieldViolations {
    /// The JSON path to the field with an invalid value
    pub field: String,
    pub violations: Vec<FieldConstraintsViolatedResponseContextFieldViolationsViolations>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConstraintsViolatedResponseContext {
    #[serde(rename = "fieldViolations")]
    pub field_violations: Vec<FieldConstraintsViolatedResponseContextFieldViolations>,
}

/// The submitted JSON entity had one or more invalid properties.
/// For each invalid field, a set of violated constraint keys are returned.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldConstraintsViolatedResponse {
    pub key: String,
    pub context: FieldConstraintsViolatedResponseContext,
}
