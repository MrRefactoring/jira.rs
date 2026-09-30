// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OptionsSettings {
    #[serde(rename = "issueContext", default, skip_serializing_if = "Option::is_none")]
    pub issue_context: Option<IssueContextParam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OptionModel>>,
}
