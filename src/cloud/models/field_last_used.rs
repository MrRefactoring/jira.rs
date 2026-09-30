// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// Last used value type:
    ///
    ///  *  *TRACKED*: field is tracked and a last used date is available.
    ///  *  *NOT\_TRACKED*: field is not tracked, last used date is not available.
    ///  *  *NO\_INFORMATION*: field is tracked, but no last used date is available.
    pub enum FieldLastUsedType {
        Tracked => "TRACKED",
        NotTracked => "NOT_TRACKED",
        NoInformation => "NO_INFORMATION",
    }
}

/// Information about the most recent use of a field.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FieldLastUsed {
    /// Last used value type:
    ///
    ///  *  *TRACKED*: field is tracked and a last used date is available.
    ///  *  *NOT\_TRACKED*: field is not tracked, last used date is not available.
    ///  *  *NO\_INFORMATION*: field is tracked, but no last used date is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<FieldLastUsedType>,
    /// The date when the value of the field last changed.
    #[cfg(feature = "chrono")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::core::deserialize_datetime",
        serialize_with = "crate::core::serialize_datetime"
    )]
    pub value: Option<chrono::DateTime<chrono::Utc>>,
    /// The date when the value of the field last changed.
    #[cfg(not(feature = "chrono"))]
    #[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::core::deserialize_timestamp")]
    pub value: Option<String>,
}
