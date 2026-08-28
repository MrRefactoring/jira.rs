// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Field with which to sort results.
    pub enum GroupSortByField {
        Name => "name",
    }
}

crate::open_enum! {
    /// Sort direction.
    pub enum GroupSortByDirection {
        Asc => "asc",
        Desc => "desc",
    }
}

/// Single sort specification for groups.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupSortBy {
    /// Field with which to sort results.
    pub field: GroupSortByField,
    /// Sort direction.
    pub direction: GroupSortByDirection,
}
