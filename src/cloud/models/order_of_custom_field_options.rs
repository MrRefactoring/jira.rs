// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The position the custom field options should be moved to. Required if `after` isn't provided.
    pub enum OrderOfCustomFieldOptionsPosition {
        First => "First",
        Last => "Last",
    }
}

/// An ordered list of custom field option IDs and information on where to move them.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OrderOfCustomFieldOptions {
    /// The ID of the custom field option or cascading option to place the moved options after. Required if `position` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A list of IDs of custom field options to move. The order of the custom field option IDs in the list is the order they are given after the move. The list must contain custom field options or cascading options, but not both.
    #[serde(rename = "customFieldOptionIds")]
    pub custom_field_option_ids: Vec<String>,
    /// The position the custom field options should be moved to. Required if `after` isn't provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<OrderOfCustomFieldOptionsPosition>,
}
