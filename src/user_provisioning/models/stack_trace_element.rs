// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StackTraceElement {
    #[serde(rename = "methodName", default, skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i64>,
    #[serde(rename = "className", default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "nativeMethod", default, skip_serializing_if = "Option::is_none")]
    pub native_method: Option<bool>,
}
