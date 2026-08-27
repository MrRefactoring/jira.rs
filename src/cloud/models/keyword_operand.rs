// @generated. Do not edit: change the generator or the specification.

use serde::{Deserialize, Serialize};

crate::open_enum! {
    /// The keyword that is the operand value.
    pub enum KeywordOperandKeyword {
        Empty => "empty",
    }
}

/// An operand that is a JQL keyword. See [Advanced searching - keywords reference](https://confluence.atlassian.com/jiracorecloud/advanced-searching-keywords-reference-765593717.html#Advancedsearching-keywordsreference-EMPTYEMPTY) for more information about operand keywords.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordOperand {
    /// The keyword that is the operand value.
    pub keyword: KeywordOperandKeyword,
}
