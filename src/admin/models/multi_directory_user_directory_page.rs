// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiDirectoryUserDirectoryPage {
    /// A page of user directory information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<MultiDirectoryUserDirectory>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<LinkPageCursor>,
}
