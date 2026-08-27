// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum ReindexType {
        Foreground => "FOREGROUND",
        Background => "BACKGROUND",
        BackgroundPreffered => "BACKGROUND_PREFFERED",
        BackgroundPreferred => "BACKGROUND_PREFERRED",
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reindex {
    #[serde(rename = "currentProgress", default, skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<i64>,
    #[serde(rename = "currentSubTask", default, skip_serializing_if = "Option::is_none")]
    pub current_sub_task: Option<String>,
    #[serde(
        rename = "finishTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub finish_time: Option<String>,
    #[serde(rename = "progressUrl", default, skip_serializing_if = "Option::is_none")]
    pub progress_url: Option<String>,
    #[serde(
        rename = "startTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub start_time: Option<String>,
    #[serde(
        rename = "submittedTime",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_timestamp"
    )]
    pub submitted_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReindexType>,
}
