// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum EstimationConfigurationValue {
        StoryPoints => "STORY_POINTS",
        OriginalEstimate => "ORIGINAL_ESTIMATE",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EstimationConfiguration {
    #[serde(rename = "localisedDescription", default, skip_serializing_if = "Option::is_none")]
    pub localised_description: Option<String>,
    #[serde(rename = "localisedName", default, skip_serializing_if = "Option::is_none")]
    pub localised_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<EstimationConfigurationValue>,
}
