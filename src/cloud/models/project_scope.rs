// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ProjectScopeAttributes {
        NotSelectable => "notSelectable",
        DefaultValue => "defaultValue",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectScope {
    /// Defines the behavior of the option in the project.If notSelectable is set, the option cannot be set as the field's value. This is useful for archiving an option that has previously been selected but shouldn't be used anymore.If defaultValue is set, the option is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<ProjectScopeAttributes>>,
    /// The ID of the project that the option's behavior applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
