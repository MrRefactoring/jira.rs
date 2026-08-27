// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

/// A list of issues and their respective properties to set or update. See [Entity properties](https://developer.atlassian.com/cloud/jira/platform/jira-entity-properties/) for more information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiIssueEntityProperties {
    /// A list of issue IDs and their respective properties.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<IssueEntityPropertiesForMultiUpdate>>,
}
