// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MultiDirectoryUserDirectory {
    /// Unique directory identifier.
    #[serde(rename = "directoryId", default, skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    /// The name of the directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the directory's icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}
