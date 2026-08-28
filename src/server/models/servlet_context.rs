// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderDefinedPackages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "specificationTitle", default, skip_serializing_if = "Option::is_none")]
    pub specification_title: Option<String>,
    #[serde(rename = "specificationVersion", default, skip_serializing_if = "Option::is_none")]
    pub specification_version: Option<String>,
    #[serde(rename = "specificationVendor", default, skip_serializing_if = "Option::is_none")]
    pub specification_vendor: Option<String>,
    #[serde(rename = "implementationTitle", default, skip_serializing_if = "Option::is_none")]
    pub implementation_title: Option<String>,
    #[serde(rename = "implementationVersion", default, skip_serializing_if = "Option::is_none")]
    pub implementation_version: Option<String>,
    #[serde(rename = "implementationVendor", default, skip_serializing_if = "Option::is_none")]
    pub implementation_vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "declaredAnnotations", default, skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderParentDefinedPackages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "specificationTitle", default, skip_serializing_if = "Option::is_none")]
    pub specification_title: Option<String>,
    #[serde(rename = "specificationVersion", default, skip_serializing_if = "Option::is_none")]
    pub specification_version: Option<String>,
    #[serde(rename = "specificationVendor", default, skip_serializing_if = "Option::is_none")]
    pub specification_vendor: Option<String>,
    #[serde(rename = "implementationTitle", default, skip_serializing_if = "Option::is_none")]
    pub implementation_title: Option<String>,
    #[serde(rename = "implementationVersion", default, skip_serializing_if = "Option::is_none")]
    pub implementation_version: Option<String>,
    #[serde(rename = "implementationVendor", default, skip_serializing_if = "Option::is_none")]
    pub implementation_vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "declaredAnnotations", default, skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sealed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderParentUnnamedModuleDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderParentUnnamedModule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "declaredAnnotations", default, skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<ServletContextClassLoaderParentUnnamedModuleDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named: Option<bool>,
    #[serde(rename = "nativeAccessEnabled", default, skip_serializing_if = "Option::is_none")]
    pub native_access_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderParent {
    #[serde(rename = "defaultAssertionStatus", default, skip_serializing_if = "Option::is_none")]
    pub default_assertion_status: Option<bool>,
    #[serde(rename = "definedPackages", default, skip_serializing_if = "Option::is_none")]
    pub defined_packages: Option<Vec<ServletContextClassLoaderParentDefinedPackages>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "registeredAsParallelCapable", default, skip_serializing_if = "Option::is_none")]
    pub registered_as_parallel_capable: Option<bool>,
    #[serde(rename = "unnamedModule", default, skip_serializing_if = "Option::is_none")]
    pub unnamed_module: Option<ServletContextClassLoaderParentUnnamedModule>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderUnnamedModuleDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoaderUnnamedModule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "declaredAnnotations", default, skip_serializing_if = "Option::is_none")]
    pub declared_annotations: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<ServletContextClassLoaderUnnamedModuleDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named: Option<bool>,
    #[serde(rename = "nativeAccessEnabled", default, skip_serializing_if = "Option::is_none")]
    pub native_access_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContextClassLoader {
    #[serde(rename = "defaultAssertionStatus", default, skip_serializing_if = "Option::is_none")]
    pub default_assertion_status: Option<bool>,
    #[serde(rename = "definedPackages", default, skip_serializing_if = "Option::is_none")]
    pub defined_packages: Option<Vec<ServletContextClassLoaderDefinedPackages>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<ServletContextClassLoaderParent>,
    #[serde(rename = "registeredAsParallelCapable", default, skip_serializing_if = "Option::is_none")]
    pub registered_as_parallel_capable: Option<bool>,
    #[serde(rename = "unnamedModule", default, skip_serializing_if = "Option::is_none")]
    pub unnamed_module: Option<ServletContextClassLoaderUnnamedModule>,
}

crate::open_enum! {
    pub enum ServletContextDefaultSessionTrackingModes {
        Cookie => "COOKIE",
        Url => "URL",
        Ssl => "SSL",
    }
}

crate::open_enum! {
    pub enum ServletContextEffectiveSessionTrackingModes {
        Cookie => "COOKIE",
        Url => "URL",
        Ssl => "SSL",
    }
}

crate::open_enum! {
    pub enum ServletContextSessionTrackingModes {
        Cookie => "COOKIE",
        Url => "URL",
        Ssl => "SSL",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ServletContext {
    #[serde(rename = "attributeNames", default, skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "classLoader", default, skip_serializing_if = "Option::is_none")]
    pub class_loader: Option<ServletContextClassLoader>,
    #[serde(rename = "contextPath", default, skip_serializing_if = "Option::is_none")]
    pub context_path: Option<String>,
    #[serde(rename = "defaultSessionTrackingModes", default, skip_serializing_if = "Option::is_none")]
    pub default_session_tracking_modes: Option<Vec<ServletContextDefaultSessionTrackingModes>>,
    #[serde(rename = "effectiveMajorVersion", default, skip_serializing_if = "Option::is_none")]
    pub effective_major_version: Option<i64>,
    #[serde(rename = "effectiveMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub effective_minor_version: Option<i64>,
    #[serde(rename = "effectiveSessionTrackingModes", default, skip_serializing_if = "Option::is_none")]
    pub effective_session_tracking_modes: Option<Vec<ServletContextEffectiveSessionTrackingModes>>,
    #[serde(rename = "filterRegistrations", default, skip_serializing_if = "Option::is_none")]
    pub filter_registrations: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "initParameterNames", default, skip_serializing_if = "Option::is_none")]
    pub init_parameter_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "jspConfigDescriptor", default, skip_serializing_if = "Option::is_none")]
    pub jsp_config_descriptor: Option<JspConfigDescriptor>,
    #[serde(rename = "majorVersion", default, skip_serializing_if = "Option::is_none")]
    pub major_version: Option<i64>,
    #[serde(rename = "minorVersion", default, skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<i64>,
    #[serde(rename = "requestCharacterEncoding", default, skip_serializing_if = "Option::is_none")]
    pub request_character_encoding: Option<String>,
    #[serde(rename = "responseCharacterEncoding", default, skip_serializing_if = "Option::is_none")]
    pub response_character_encoding: Option<String>,
    #[serde(rename = "serverInfo", default, skip_serializing_if = "Option::is_none")]
    pub server_info: Option<String>,
    #[serde(rename = "servletContextName", default, skip_serializing_if = "Option::is_none")]
    pub servlet_context_name: Option<String>,
    #[serde(rename = "servletRegistrations", default, skip_serializing_if = "Option::is_none")]
    pub servlet_registrations: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "sessionCookieConfig", default, skip_serializing_if = "Option::is_none")]
    pub session_cookie_config: Option<SessionCookieConfig>,
    #[serde(rename = "sessionTimeout", default, skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i64>,
    #[serde(rename = "sessionTrackingModes", default, skip_serializing_if = "Option::is_none")]
    pub session_tracking_modes: Option<Vec<ServletContextSessionTrackingModes>>,
    #[serde(rename = "virtualServerName", default, skip_serializing_if = "Option::is_none")]
    pub virtual_server_name: Option<String>,
}
