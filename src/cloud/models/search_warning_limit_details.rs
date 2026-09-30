// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// Experimental. Structured details about a JQL clause exceeding its argument limit.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchWarningLimitDetails {
    /// The actual number of arguments supplied that exceeded the limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual: Option<i64>,
    /// The arguments passed to the JQL clause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    /// The JQL clause that triggered the limit, e.g. issueHistory().
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clause: Option<String>,
    /// The maximum number of arguments allowed for the clause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
