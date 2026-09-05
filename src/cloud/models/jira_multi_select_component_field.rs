// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum JiraMultiSelectComponentFieldBulkEditMultiSelectFieldOption {
        Add => "ADD",
        Remove => "REMOVE",
        Replace => "REPLACE",
        RemoveAll => "REMOVE_ALL",
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JiraMultiSelectComponentField {
    #[serde(rename = "bulkEditMultiSelectFieldOption")]
    pub bulk_edit_multi_select_field_option: JiraMultiSelectComponentFieldBulkEditMultiSelectFieldOption,
    pub components: Vec<JiraComponentField>,
    #[serde(rename = "fieldId")]
    pub field_id: String,
}
