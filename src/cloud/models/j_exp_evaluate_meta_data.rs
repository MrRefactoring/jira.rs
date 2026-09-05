// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// Contains information about the expression evaluation. This bean will be replacing `JiraExpressionEvaluationMetaDataBean` bean as part of new `evaluate` endpoint
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JExpEvaluateMetaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub complexity: Option<JiraExpressionsComplexity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<JExpEvaluateIssuesMeta>,
}
