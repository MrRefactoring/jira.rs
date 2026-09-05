// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    pub enum JsonNodeNumberType {
        Int => "INT",
        Long => "LONG",
        BigInteger => "BIG_INTEGER",
        Float => "FLOAT",
        Double => "DOUBLE",
        BigDecimal => "BIG_DECIMAL",
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct JsonNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<bool>,
    #[serde(rename = "bigDecimal", default, skip_serializing_if = "Option::is_none")]
    pub big_decimal: Option<bool>,
    #[serde(rename = "bigInteger", default, skip_serializing_if = "Option::is_none")]
    pub big_integer: Option<bool>,
    #[serde(rename = "bigIntegerValue", default, skip_serializing_if = "Option::is_none")]
    pub big_integer_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename = "binaryValue", default, skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(rename = "booleanValue", default, skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "containerNode", default, skip_serializing_if = "Option::is_none")]
    pub container_node: Option<bool>,
    #[serde(rename = "decimalValue", default, skip_serializing_if = "Option::is_none")]
    pub decimal_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double: Option<bool>,
    #[serde(rename = "doubleValue", default, skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "fieldNames", default, skip_serializing_if = "Option::is_none")]
    pub field_names: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "floatingPointNumber", default, skip_serializing_if = "Option::is_none")]
    pub floating_point_number: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub int: Option<bool>,
    #[serde(rename = "intValue", default, skip_serializing_if = "Option::is_none")]
    pub int_value: Option<i64>,
    #[serde(rename = "integralNumber", default, skip_serializing_if = "Option::is_none")]
    pub integral_number: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub long: Option<bool>,
    #[serde(rename = "longValue", default, skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    #[serde(rename = "missingNode", default, skip_serializing_if = "Option::is_none")]
    pub missing_node: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<bool>,
    #[serde(rename = "numberType", default, skip_serializing_if = "Option::is_none")]
    pub number_type: Option<JsonNodeNumberType>,
    #[serde(rename = "numberValue", default, skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pojo: Option<bool>,
    #[serde(rename = "textValue", default, skip_serializing_if = "Option::is_none")]
    pub text_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub textual: Option<bool>,
    #[serde(rename = "valueAsBoolean", default, skip_serializing_if = "Option::is_none")]
    pub value_as_boolean: Option<bool>,
    #[serde(rename = "valueAsDouble", default, skip_serializing_if = "Option::is_none")]
    pub value_as_double: Option<f64>,
    #[serde(rename = "valueAsInt", default, skip_serializing_if = "Option::is_none")]
    pub value_as_int: Option<i64>,
    #[serde(rename = "valueAsLong", default, skip_serializing_if = "Option::is_none")]
    pub value_as_long: Option<i64>,
    #[serde(rename = "valueAsText", default, skip_serializing_if = "Option::is_none")]
    pub value_as_text: Option<String>,
    #[serde(rename = "valueNode", default, skip_serializing_if = "Option::is_none")]
    pub value_node: Option<bool>,
}
