// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum HttpServletRequestDispatcherType {
        Forward => "FORWARD",
        Include => "INCLUDE",
        Request => "REQUEST",
        Async => "ASYNC",
        Error => "ERROR",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HttpServletRequestLocale {
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HttpServletRequestRequestURL {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HttpServletRequestUserPrincipal {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HttpServletRequest {
    #[serde(rename = "asyncContext", default, skip_serializing_if = "Option::is_none")]
    pub async_context: Option<AsyncContext>,
    #[serde(rename = "asyncStarted", default, skip_serializing_if = "Option::is_none")]
    pub async_started: Option<bool>,
    #[serde(rename = "asyncSupported", default, skip_serializing_if = "Option::is_none")]
    pub async_supported: Option<bool>,
    #[serde(rename = "attributeNames", default, skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "authType", default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "characterEncoding", default, skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[serde(rename = "contentLength", default, skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "contentLengthLong", default, skip_serializing_if = "Option::is_none")]
    pub content_length_long: Option<i64>,
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cookies: Option<Vec<Cookie>>,
    #[serde(rename = "dispatcherType", default, skip_serializing_if = "Option::is_none")]
    pub dispatcher_type: Option<HttpServletRequestDispatcherType>,
    #[serde(rename = "headerNames", default, skip_serializing_if = "Option::is_none")]
    pub header_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "httpServletMapping", default, skip_serializing_if = "Option::is_none")]
    pub http_servlet_mapping: Option<HttpServletMapping>,
    #[serde(rename = "inputStream", default, skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<ServletInputStream>,
    #[serde(rename = "localAddr", default, skip_serializing_if = "Option::is_none")]
    pub local_addr: Option<String>,
    #[serde(rename = "localName", default, skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(rename = "localPort", default, skip_serializing_if = "Option::is_none")]
    pub local_port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<HttpServletRequestLocale>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locales: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "parameterMap", default, skip_serializing_if = "Option::is_none")]
    pub parameter_map: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "parameterNames", default, skip_serializing_if = "Option::is_none")]
    pub parameter_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[serde(rename = "pathInfo", default, skip_serializing_if = "Option::is_none")]
    pub path_info: Option<String>,
    #[serde(rename = "pathTranslated", default, skip_serializing_if = "Option::is_none")]
    pub path_translated: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "protocolRequestId", default, skip_serializing_if = "Option::is_none")]
    pub protocol_request_id: Option<String>,
    #[serde(rename = "queryString", default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reader: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "remoteAddr", default, skip_serializing_if = "Option::is_none")]
    pub remote_addr: Option<String>,
    #[serde(rename = "remoteHost", default, skip_serializing_if = "Option::is_none")]
    pub remote_host: Option<String>,
    #[serde(rename = "remotePort", default, skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i64>,
    #[serde(rename = "remoteUser", default, skip_serializing_if = "Option::is_none")]
    pub remote_user: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "requestURI", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(rename = "requestURL", default, skip_serializing_if = "Option::is_none")]
    pub request_url: Option<HttpServletRequestRequestURL>,
    #[serde(rename = "requestedSessionId", default, skip_serializing_if = "Option::is_none")]
    pub requested_session_id: Option<String>,
    #[serde(rename = "requestedSessionIdFromCookie", default, skip_serializing_if = "Option::is_none")]
    pub requested_session_id_from_cookie: Option<bool>,
    #[serde(rename = "requestedSessionIdFromURL", default, skip_serializing_if = "Option::is_none")]
    pub requested_session_id_from_url: Option<bool>,
    #[serde(rename = "requestedSessionIdValid", default, skip_serializing_if = "Option::is_none")]
    pub requested_session_id_valid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(rename = "serverName", default, skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "serverPort", default, skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i64>,
    #[serde(rename = "servletConnection", default, skip_serializing_if = "Option::is_none")]
    pub servlet_connection: Option<ServletConnection>,
    #[serde(rename = "servletContext", default, skip_serializing_if = "Option::is_none")]
    pub servlet_context: Option<ServletContext>,
    #[serde(rename = "servletPath", default, skip_serializing_if = "Option::is_none")]
    pub servlet_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<HttpSession>,
    #[serde(rename = "trailerFields", default, skip_serializing_if = "Option::is_none")]
    pub trailer_fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "trailerFieldsReady", default, skip_serializing_if = "Option::is_none")]
    pub trailer_fields_ready: Option<bool>,
    #[serde(rename = "userPrincipal", default, skip_serializing_if = "Option::is_none")]
    pub user_principal: Option<HttpServletRequestUserPrincipal>,
}
