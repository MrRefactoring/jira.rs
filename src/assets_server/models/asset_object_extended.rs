// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AssetObjectExtended {
    #[serde(rename = "openIssuesExists", default, skip_serializing_if = "Option::is_none")]
    pub open_issues_exists: Option<bool>,
    #[serde(rename = "attachmentsExists", default, skip_serializing_if = "Option::is_none")]
    pub attachments_exists: Option<bool>,
}
