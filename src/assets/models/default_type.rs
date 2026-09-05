// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum DefaultTypeVariant0Id {
        N1Descending => "-1",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant0Name {
    #[serde(rename = "None")]
    None,
}

crate::open_enum! {
    pub enum DefaultTypeVariant1Id {
        N0 => "0",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant1Name {
    #[serde(rename = "Text")]
    Text,
}

crate::open_enum! {
    pub enum DefaultTypeVariant2Id {
        N1 => "1",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant2Name {
    #[serde(rename = "Integer")]
    Integer,
}

crate::open_enum! {
    pub enum DefaultTypeVariant3Id {
        N2 => "2",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant3Name {
    #[serde(rename = "Boolean")]
    Boolean,
}

crate::open_enum! {
    pub enum DefaultTypeVariant4Id {
        N3 => "3",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant4Name {
    #[serde(rename = "Double")]
    Double,
}

crate::open_enum! {
    pub enum DefaultTypeVariant5Id {
        N4 => "4",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant5Name {
    #[serde(rename = "Date")]
    Date,
}

crate::open_enum! {
    pub enum DefaultTypeVariant6Id {
        N5 => "5",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant6Name {
    #[serde(rename = "Time")]
    Time,
}

crate::open_enum! {
    pub enum DefaultTypeVariant7Id {
        N6 => "6",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant7Name {
    #[serde(rename = "DateTime")]
    DateTime,
}

crate::open_enum! {
    pub enum DefaultTypeVariant8Id {
        N7 => "7",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant8Name {
    #[serde(rename = "Url")]
    Url,
}

crate::open_enum! {
    pub enum DefaultTypeVariant9Id {
        N8 => "8",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant9Name {
    #[serde(rename = "Email")]
    Email,
}

crate::open_enum! {
    pub enum DefaultTypeVariant10Id {
        N9 => "9",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant10Name {
    #[serde(rename = "Textarea")]
    Textarea,
}

crate::open_enum! {
    pub enum DefaultTypeVariant11Id {
        N10 => "10",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant11Name {
    #[serde(rename = "Select")]
    Select,
}

crate::open_enum! {
    pub enum DefaultTypeVariant12Id {
        N11 => "11",
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefaultTypeVariant12Name {
    #[serde(rename = "IP Address")]
    IpAddress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue {
    pub id: DefaultTypeVariant0Id,
    pub name: DefaultTypeVariant0Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue2 {
    pub id: DefaultTypeVariant1Id,
    pub name: DefaultTypeVariant1Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue3 {
    pub id: DefaultTypeVariant2Id,
    pub name: DefaultTypeVariant2Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue4 {
    pub id: DefaultTypeVariant3Id,
    pub name: DefaultTypeVariant3Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue5 {
    pub id: DefaultTypeVariant4Id,
    pub name: DefaultTypeVariant4Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue6 {
    pub id: DefaultTypeVariant5Id,
    pub name: DefaultTypeVariant5Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue7 {
    pub id: DefaultTypeVariant6Id,
    pub name: DefaultTypeVariant6Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue8 {
    pub id: DefaultTypeVariant7Id,
    pub name: DefaultTypeVariant7Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue9 {
    pub id: DefaultTypeVariant8Id,
    pub name: DefaultTypeVariant8Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue10 {
    pub id: DefaultTypeVariant9Id,
    pub name: DefaultTypeVariant9Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue11 {
    pub id: DefaultTypeVariant10Id,
    pub name: DefaultTypeVariant10Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue12 {
    pub id: DefaultTypeVariant11Id,
    pub name: DefaultTypeVariant11Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultTypeValue13 {
    pub id: DefaultTypeVariant12Id,
    pub name: DefaultTypeVariant12Name,
}

/// | Id | Description |
/// | -- | ----------- |
/// | -1 | None |
/// | 0 | Text |
/// | 1 | Integer |
/// | 2 | Boolean |
/// | 3 | Double |
/// | 4 | Date |
/// | 5 | Time |
/// | 6 | DateTime |
/// | 7 | Url |
/// | 8 | Email |
/// | 9 | Textarea |
/// | 10 | Select |
/// | 11 | IP Address |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum DefaultType {
    Variant0(DefaultTypeValue),
    Variant1(DefaultTypeValue2),
    Variant2(DefaultTypeValue3),
    Variant3(DefaultTypeValue4),
    Variant4(DefaultTypeValue5),
    Variant5(DefaultTypeValue6),
    Variant6(DefaultTypeValue7),
    Variant7(DefaultTypeValue8),
    Variant8(DefaultTypeValue9),
    Variant9(DefaultTypeValue10),
    Variant10(DefaultTypeValue11),
    Variant11(DefaultTypeValue12),
    Variant12(DefaultTypeValue13),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
