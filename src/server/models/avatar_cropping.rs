// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvatarCropping {
    #[serde(rename = "cropperOffsetX", default, skip_serializing_if = "Option::is_none")]
    pub cropper_offset_x: Option<i64>,
    #[serde(rename = "cropperOffsetY", default, skip_serializing_if = "Option::is_none")]
    pub cropper_offset_y: Option<i64>,
    #[serde(rename = "cropperWidth", default, skip_serializing_if = "Option::is_none")]
    pub cropper_width: Option<i64>,
    #[serde(rename = "needsCropping", default, skip_serializing_if = "Option::is_none")]
    pub needs_cropping: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
