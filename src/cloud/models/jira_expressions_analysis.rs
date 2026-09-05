// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about the analysed Jira expression.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JiraExpressionsAnalysis {
    /// The results of Jira expressions analysis.
    pub results: Vec<JiraExpressionAnalysis>,
}
