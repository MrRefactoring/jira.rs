// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServletResponseLocale {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "displayCountry", default, skip_serializing_if = "Option::is_none")]
    pub display_country: Option<String>,
    #[serde(rename = "displayLanguage", default, skip_serializing_if = "Option::is_none")]
    pub display_language: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "displayScript", default, skip_serializing_if = "Option::is_none")]
    pub display_script: Option<String>,
    #[serde(rename = "displayVariant", default, skip_serializing_if = "Option::is_none")]
    pub display_variant: Option<String>,
    #[serde(rename = "extensionKeys", default, skip_serializing_if = "Option::is_none")]
    pub extension_keys: Option<Vec<String>>,
    #[serde(rename = "iso3Country", default, skip_serializing_if = "Option::is_none")]
    pub iso3_country: Option<String>,
    #[serde(rename = "iso3Language", default, skip_serializing_if = "Option::is_none")]
    pub iso3_language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(rename = "unicodeLocaleAttributes", default, skip_serializing_if = "Option::is_none")]
    pub unicode_locale_attributes: Option<Vec<String>>,
    #[serde(rename = "unicodeLocaleKeys", default, skip_serializing_if = "Option::is_none")]
    pub unicode_locale_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServletResponse {
    #[serde(rename = "bufferSize", default, skip_serializing_if = "Option::is_none")]
    pub buffer_size: Option<i64>,
    #[serde(rename = "characterEncoding", default, skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed: Option<bool>,
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "contentLengthLong", default, skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<ServletResponseLocale>,
    #[serde(rename = "outputStream", default, skip_serializing_if = "Option::is_none")]
    pub output_stream: Option<ServletOutputStream>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub writer: Option<std::collections::HashMap<String, serde_json::Value>>,
}
