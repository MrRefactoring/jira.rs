// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JiraExpressionsComplexity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub beans: Option<JiraExpressionsComplexityValue>,
    #[serde(rename = "expensiveOperations", default, skip_serializing_if = "Option::is_none")]
    pub expensive_operations: Option<JiraExpressionsComplexityValue>,
    #[serde(rename = "primitiveValues", default, skip_serializing_if = "Option::is_none")]
    pub primitive_values: Option<JiraExpressionsComplexityValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<JiraExpressionsComplexityValue>,
}
