// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum IssueFieldOptionConfigurationAttributes {
        NotSelectable => "notSelectable",
        DefaultValue => "defaultValue",
    }
}

/// Details of the projects the option is available in.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueFieldOptionConfiguration {
    /// DEPRECATED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<IssueFieldOptionConfigurationAttributes>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<IssueFieldOptionScope>,
}
