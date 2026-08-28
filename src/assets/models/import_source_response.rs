// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Import module type. CSV: rlabs-import-type-csv, JSON: rlabs-import-type-json, External: rlabs-import-type-external, Discovery: insight-discovery-import, DataManager: rlabs-import-type-dm-csv
    pub enum ImportSourceResponseImportSourceModuleKey {
        RlabsImportTypeCsv => "rlabs-import-type-csv",
        RlabsImportTypeJson => "rlabs-import-type-json",
        RlabsImportTypeExternal => "rlabs-import-type-external",
        InsightDiscoveryImport => "insight-discovery-import",
        RlabsImportTypeDmCsv => "rlabs-import-type-dm-csv",
    }
}

crate::open_enum! {
    /// Configuration status type - whether the import source is enabled or disabled
    pub enum ImportSourceResponseImportStatusConfigurationStatusType {
        Disabled => "DISABLED",
        Enabled => "ENABLED",
    }
}

crate::open_enum! {
    /// Validation status type - system-evaluated status (not user-changeable)
    pub enum ImportSourceResponseImportStatusValidationStatusType {
        Valid => "VALID",
        InvalidConfiguration => "INVALID_CONFIGURATION",
        ModuleUninstalled => "MODULE_UNINSTALLED",
    }
}

/// Import status information
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponseImportStatus {
    /// Configuration status type - whether the import source is enabled or disabled
    #[serde(rename = "configurationStatusType", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status_type: Option<ImportSourceResponseImportStatusConfigurationStatusType>,
    /// Validation status type - system-evaluated status (not user-changeable)
    #[serde(rename = "validationStatusType", default, skip_serializing_if = "Option::is_none")]
    pub validation_status_type: Option<ImportSourceResponseImportStatusValidationStatusType>,
    /// Map of reasons for invalidity
    #[serde(rename = "reasonForInvalidity", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_invalidity: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Status name (computed from configurationStatusType)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Validation status name (computed)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    /// AUI lozenge CSS class for configuration status
    #[serde(rename = "configurationAuiLozenge", default, skip_serializing_if = "Option::is_none")]
    pub configuration_aui_lozenge: Option<String>,
    /// AUI lozenge CSS class for validation status
    #[serde(rename = "validationAuiLozenge", default, skip_serializing_if = "Option::is_none")]
    pub validation_aui_lozenge: Option<String>,
}

/// Target object type configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponseImportSourceOTEntriesObjectType {
    /// Object type ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Object type name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

crate::open_enum! {
    /// Configuration status type
    pub enum ImportSourceResponseImportSourceOTEntriesImportStatusConfigurationStatusType {
        Disabled => "DISABLED",
        Enabled => "ENABLED",
    }
}

crate::open_enum! {
    /// Validation status type
    pub enum ImportSourceResponseImportSourceOTEntriesImportStatusValidationStatusType {
        Valid => "VALID",
        InvalidConfiguration => "INVALID_CONFIGURATION",
        ModuleUninstalled => "MODULE_UNINSTALLED",
    }
}

/// Import status for this object type
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponseImportSourceOTEntriesImportStatus {
    /// Configuration status type
    #[serde(rename = "configurationStatusType", default, skip_serializing_if = "Option::is_none")]
    pub configuration_status_type: Option<ImportSourceResponseImportSourceOTEntriesImportStatusConfigurationStatusType>,
    /// Validation status type
    #[serde(rename = "validationStatusType", default, skip_serializing_if = "Option::is_none")]
    pub validation_status_type: Option<ImportSourceResponseImportSourceOTEntriesImportStatusValidationStatusType>,
    /// Reasons for invalidity
    #[serde(rename = "reasonForInvalidity", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_invalidity: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponseImportSourceOTEntriesImportSourceOTAttrEntries {
    /// Attribute mapping ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Import source object type configuration
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponseImportSourceOTEntries {
    /// Import source object type ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Parent import source object type ID
    #[serde(rename = "parentImportSourceOTId", default, skip_serializing_if = "Option::is_none")]
    pub parent_import_source_ot_id: Option<String>,
    /// Associated import source ID
    #[serde(rename = "importSourceId", default, skip_serializing_if = "Option::is_none")]
    pub import_source_id: Option<String>,
    /// Creation timestamp
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Creation timestamp
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// Last update timestamp
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// Last update timestamp
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    /// Description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Target object type configuration
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ImportSourceResponseImportSourceOTEntriesObjectType>,
    /// Selector QL query
    #[serde(rename = "selectorQlQuery", default, skip_serializing_if = "Option::is_none")]
    pub selector_ql_query: Option<String>,
    /// Selector IQL query
    #[serde(rename = "selectorIQL", default, skip_serializing_if = "Option::is_none")]
    pub selector_iql: Option<String>,
    /// The selector used in JSON imports to find the objects
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// How to handle empty values
    #[serde(rename = "emptyValues", default, skip_serializing_if = "Option::is_none")]
    pub empty_values: Option<String>,
    /// How to handle unknown values
    #[serde(rename = "unknownValues", default, skip_serializing_if = "Option::is_none")]
    pub unknown_values: Option<String>,
    /// Import status for this object type
    #[serde(rename = "importStatus", default, skip_serializing_if = "Option::is_none")]
    pub import_status: Option<ImportSourceResponseImportSourceOTEntriesImportStatus>,
    /// List of object type attribute mappings
    #[serde(rename = "importSourceOTAttrEntries", default, skip_serializing_if = "Option::is_none")]
    pub import_source_ot_attr_entries: Option<Vec<ImportSourceResponseImportSourceOTEntriesImportSourceOTAttrEntries>>,
    /// Whether to ignore case when matching identifiers
    #[serde(rename = "matchIdentifierIgnoreCase", default, skip_serializing_if = "Option::is_none")]
    pub match_identifier_ignore_case: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ImportSourceResponse {
    /// Import source ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Collection (object schema) ID
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// Import source name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Timestamp when the import source was created
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp when the import source was created
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub created: Option<String>,
    /// Timestamp when the import source was last updated
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
    /// Timestamp when the import source was last updated
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub updated: Option<String>,
    /// Import source description
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Object schema ID
    #[serde(rename = "objectSchemaId", default, skip_serializing_if = "Option::is_none")]
    pub object_schema_id: Option<String>,
    /// Import module type. CSV: rlabs-import-type-csv, JSON: rlabs-import-type-json, External: rlabs-import-type-external, Discovery: insight-discovery-import, DataManager: rlabs-import-type-dm-csv
    #[serde(rename = "importSourceModuleKey", default, skip_serializing_if = "Option::is_none")]
    pub import_source_module_key: Option<ImportSourceResponseImportSourceModuleKey>,
    /// Default concatenator for multi-value attributes
    #[serde(rename = "defaultConcatenator", default, skip_serializing_if = "Option::is_none")]
    pub default_concatenator: Option<String>,
    /// How to handle empty values
    #[serde(rename = "defaultHandleEmptyValues", default, skip_serializing_if = "Option::is_none")]
    pub default_handle_empty_values: Option<String>,
    /// How to handle unknown values
    #[serde(rename = "defaultHandleUnknownValues", default, skip_serializing_if = "Option::is_none")]
    pub default_handle_unknown_values: Option<String>,
    /// Date format pattern
    #[serde(rename = "dateFormat", default, skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    /// Date-time format pattern
    #[serde(rename = "dateTimeFormat", default, skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
    /// Import status information
    #[serde(rename = "importStatus", default, skip_serializing_if = "Option::is_none")]
    pub import_status: Option<ImportSourceResponseImportStatus>,
    /// Import-specific configuration as JSON string
    #[serde(rename = "importSpecificConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub import_specific_configuration: Option<String>,
    /// List of object type mappings for this import source
    #[serde(rename = "importSourceOTEntries", default, skip_serializing_if = "Option::is_none")]
    pub import_source_ot_entries: Option<Vec<ImportSourceResponseImportSourceOTEntries>>,
    /// Whether a token has been generated for this import source
    #[serde(rename = "tokenGenerated", default, skip_serializing_if = "Option::is_none")]
    pub token_generated: Option<bool>,
    /// Import source URL (if applicable)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Integrated import type ID
    #[serde(rename = "integratedImportTypeId", default, skip_serializing_if = "Option::is_none")]
    pub integrated_import_type_id: Option<i64>,
    /// Integrated import type extension ID
    #[serde(rename = "integratedImportTypeExtensionId", default, skip_serializing_if = "Option::is_none")]
    pub integrated_import_type_extension_id: Option<String>,
    /// How to handle computed issue values (DataManager config)
    #[serde(rename = "defaultHandleComputeIssueValues", default, skip_serializing_if = "Option::is_none")]
    pub default_handle_compute_issue_values: Option<String>,
    /// How to handle null values (DataManager config)
    #[serde(rename = "defaultHandleNullValues", default, skip_serializing_if = "Option::is_none")]
    pub default_handle_null_values: Option<String>,
    /// How to handle not mapped values (DataManager config)
    #[serde(rename = "defaultHandleNotMappedValues", default, skip_serializing_if = "Option::is_none")]
    pub default_handle_not_mapped_values: Option<String>,
    /// Whether scheduled import is enabled for this source
    #[serde(rename = "isImportSourceSchedulingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_import_source_scheduling_enabled: Option<bool>,
    /// Type of import execution
    #[serde(rename = "importExecutionType", default, skip_serializing_if = "Option::is_none")]
    pub import_execution_type: Option<String>,
    #[serde(rename = "scheduledImportDetails", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_import_details: Option<ScheduledImportDetails>,
}
