// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum GlobalScopeAttributes {
        NotSelectable => "notSelectable",
        DefaultValue => "defaultValue",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GlobalScope {
    /// Defines the behavior of the option in the global context.If notSelectable is set, the option cannot be set as the field's value. This is useful for archiving an option that has previously been selected but shouldn't be used anymore.If defaultValue is set, the option is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<GlobalScopeAttributes>>,
}
