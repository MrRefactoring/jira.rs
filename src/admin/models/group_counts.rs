// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// The number of objects associated with the group.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCounts {
    /// The number of users that belong to the group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<i64>,
    /// The number of resources the group has roles assigned to, linked to the  directories the requestor can manage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<i64>,
}
