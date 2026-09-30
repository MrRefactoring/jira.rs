// @generated. Do not edit: change the generator or the specification.

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CustomFieldContextDefaultValue {
    CustomFieldContextDefaultValueCascadingOption(CustomFieldContextDefaultValueCascadingOption),
    CustomFieldContextDefaultValueMultipleOption(CustomFieldContextDefaultValueMultipleOption),
    CustomFieldContextDefaultValueSingleOption(CustomFieldContextDefaultValueSingleOption),
    CustomFieldContextSingleUserPickerDefaults(CustomFieldContextSingleUserPickerDefaults),
    CustomFieldContextDefaultValueMultiUserPicker(CustomFieldContextDefaultValueMultiUserPicker),
    CustomFieldContextDefaultValueSingleGroupPicker(CustomFieldContextDefaultValueSingleGroupPicker),
    CustomFieldContextDefaultValueMultipleGroupPicker(CustomFieldContextDefaultValueMultipleGroupPicker),
    CustomFieldContextDefaultValueDate(CustomFieldContextDefaultValueDate),
    CustomFieldContextDefaultValueDateTime(CustomFieldContextDefaultValueDateTime),
    CustomFieldContextDefaultValueURL(CustomFieldContextDefaultValueURL),
    CustomFieldContextDefaultValueProject(CustomFieldContextDefaultValueProject),
    CustomFieldContextDefaultValueFloat(CustomFieldContextDefaultValueFloat),
    CustomFieldContextDefaultValueLabels(CustomFieldContextDefaultValueLabels),
    CustomFieldContextDefaultValueTextField(CustomFieldContextDefaultValueTextField),
    CustomFieldContextDefaultValueTextArea(CustomFieldContextDefaultValueTextArea),
    CustomFieldContextDefaultValueReadOnly(CustomFieldContextDefaultValueReadOnly),
    CustomFieldContextDefaultValueSingleVersionPicker(CustomFieldContextDefaultValueSingleVersionPicker),
    CustomFieldContextDefaultValueMultipleVersionPicker(CustomFieldContextDefaultValueMultipleVersionPicker),
    CustomFieldContextDefaultValueForgeStringField(CustomFieldContextDefaultValueForgeStringField),
    CustomFieldContextDefaultValueForgeMultiStringField(CustomFieldContextDefaultValueForgeMultiStringField),
    CustomFieldContextDefaultValueForgeObjectField(CustomFieldContextDefaultValueForgeObjectField),
    CustomFieldContextDefaultValueForgeDateTimeField(CustomFieldContextDefaultValueForgeDateTimeField),
    CustomFieldContextDefaultValueForgeGroupField(CustomFieldContextDefaultValueForgeGroupField),
    CustomFieldContextDefaultValueForgeMultiGroupField(CustomFieldContextDefaultValueForgeMultiGroupField),
    CustomFieldContextDefaultValueForgeNumberField(CustomFieldContextDefaultValueForgeNumberField),
    CustomFieldContextDefaultValueForgeUserField(CustomFieldContextDefaultValueForgeUserField),
    CustomFieldContextDefaultValueForgeMultiUserField(CustomFieldContextDefaultValueForgeMultiUserField),
    /// A shape the specification does not describe.
    Other(serde_json::Value),
}
