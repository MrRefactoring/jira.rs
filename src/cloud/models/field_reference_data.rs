// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Whether the field provide auto-complete suggestions.
    pub enum FieldReferenceDataAuto {
        True => "true",
        False => "false",
    }
}

crate::open_enum! {
    /// Whether this field has been deprecated.
    pub enum FieldReferenceDataDeprecated {
        True => "true",
        False => "false",
    }
}

crate::open_enum! {
    /// Whether the field can be used in a query's `ORDER BY` clause.
    pub enum FieldReferenceDataOrderable {
        True => "true",
        False => "false",
    }
}

crate::open_enum! {
    /// Whether the content of this field can be searched.
    pub enum FieldReferenceDataSearchable {
        True => "true",
        False => "false",
    }
}

/// Details of a field that can be used in advanced searches.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FieldReferenceData {
    /// Whether the field provide auto-complete suggestions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<FieldReferenceDataAuto>,
    /// If the item is a custom field, the ID of the custom field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cfid: Option<String>,
    /// Whether this field has been deprecated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<FieldReferenceDataDeprecated>,
    /// The searcher key of the field, only passed when the field is deprecated.
    #[serde(rename = "deprecatedSearcherKey", default, skip_serializing_if = "Option::is_none")]
    pub deprecated_searcher_key: Option<String>,
    /// The display name contains the following:
    ///
    ///  *  for system fields, the field name. For example, `Summary`.
    ///  *  for collapsed custom fields, the field name followed by a hyphen and then the field name and field type. For example, `Component - Component[Dropdown]`.
    ///  *  for other custom fields, the field name followed by a hyphen and then the custom field ID. For example, `Component - cf[10061]`.
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The valid search operators for the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operators: Option<Vec<String>>,
    /// Whether the field can be used in a query's `ORDER BY` clause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orderable: Option<FieldReferenceDataOrderable>,
    /// Whether the content of this field can be searched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<FieldReferenceDataSearchable>,
    /// The data types of items in the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// The field identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
