// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Details about the analysed Jira expression.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JiraExpressionsAnalysis {
    /// The results of Jira expressions analysis.
    pub results: Vec<JiraExpressionAnalysis>,
}
