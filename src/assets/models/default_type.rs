// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

/// | Id | Description |
/// | -- | ----------- |
/// | -1 | None |
/// | 0 | Text |
/// | 1 | Integer |
/// | 2 | Boolean |
/// | 3 | Double |
/// | 4 | Date |
/// | 5 | Time |
/// | 6 | DateTime |
/// | 7 | Url |
/// | 8 | Email |
/// | 9 | Textarea |
/// | 10 | Select |
/// | 11 | IP Address |
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultType {
    pub id: i64,
    pub name: String,
}
