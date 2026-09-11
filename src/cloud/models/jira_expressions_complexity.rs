// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JiraExpressionsComplexity {
    pub beans: JiraExpressionsComplexityValue,
    #[serde(rename = "expensiveOperations")]
    pub expensive_operations: JiraExpressionsComplexityValue,
    #[serde(rename = "primitiveValues")]
    pub primitive_values: JiraExpressionsComplexityValue,
    pub steps: JiraExpressionsComplexityValue,
}
