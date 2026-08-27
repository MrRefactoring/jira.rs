// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum JiraLabelsFieldBulkEditMultiSelectFieldOption {
        Add => "ADD",
        Remove => "REMOVE",
        Replace => "REPLACE",
        RemoveAll => "REMOVE_ALL",
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraLabelsField {
    #[serde(rename = "bulkEditMultiSelectFieldOption")]
    pub bulk_edit_multi_select_field_option: JiraLabelsFieldBulkEditMultiSelectFieldOption,
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "labelProperties", default, skip_serializing_if = "Option::is_none")]
    pub label_properties: Option<Vec<JiraLabelProperties>>,
    pub labels: Vec<JiraLabelsInput>,
}
