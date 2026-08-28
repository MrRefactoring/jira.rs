//! Building a JQL query out of values that came from somewhere else.
//!
//! A query written with [`format!`] is a query a value can escape from: a quotation mark in a summary someone typed
//! ends the literal, and the rest of what they typed is read as query. Everything here quotes what it is given and
//! puts the operators in itself, so a value stays a value.
//!
//! ```
//! use jira::jql::{field, func};
//!
//! let query = field("project").eq("PROJ")
//!     .and(field("summary").contains("say \"hello\""))
//!     .and(field("status").not_in(["Done", "Closed"]))
//!     .and(field("assignee").eq(func("currentUser")))
//!     .order_by_desc("created");
//!
//! assert_eq!(
//!     query.to_string(),
//!     r#"project = "PROJ" AND summary ~ "say \"hello\"" AND status NOT IN ("Done", "Closed") "#.to_owned()
//!         + r#"AND assignee = currentUser() ORDER BY created DESC"#,
//! );
//! ```
//!
//! A [`Query`] and a [`Clause`] both go straight to an operation that takes one, so the query above needs no
//! rendering of its own:
//!
//! ```no_run
//! # use jira::cloud::CloudClient;
//! # use jira::jql::field;
//! # async fn example(jira: CloudClient) -> jira::Result<()> {
//! let page = jira
//!     .issue_search()
//!     .search_issues()
//!     .jql(field("project").eq("PROJ").order_by_desc("created"))
//!     .fields(["summary"])
//!     .send()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! What is not here is a model of JQL itself. `WAS`, `CHANGED`, `DURING` and the rest of the history operators are
//! reachable through [`raw`], which puts a fragment in unescaped and is the one place a caller is responsible for
//! what it contains.

use std::fmt::{self, Display, Formatter};

/// The name of a field a clause is about.
///
/// A name that is a plain word reaches JQL as it was written; anything else is quoted, so a field named
/// `Story Points` and a custom field written `cf[10001]` both work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field(String);

/// One side of a comparison: text, a number, a function call, or a fragment written by hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(String);

/// A condition, or several joined by `AND` and `OR`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause(Node);

/// A clause with the order its results come back in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    clause: Clause,
    order: Vec<(Field, Direction)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Comparison(String),
    And(Vec<Node>),
    Or(Vec<Node>),
    Not(Box<Node>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Ascending,
    Descending,
}

/// Names the field a clause is about.
pub fn field(name: impl Into<String>) -> Field {
    Field(name.into())
}

/// A JQL function with no arguments, such as `currentUser()`.
pub fn func(name: impl Into<String>) -> Value {
    Value(format!("{}()", name.into()))
}

/// A JQL function with arguments, each quoted the way a value is: `membersOf("jira-administrators")`.
pub fn func_with(name: impl Into<String>, arguments: impl IntoIterator<Item = impl Into<Value>>) -> Value {
    let arguments = arguments.into_iter().map(|argument| argument.into().0).collect::<Vec<_>>().join(", ");

    Value(format!("{}({arguments})", name.into()))
}

/// A fragment put into the query exactly as written, escaping nothing.
///
/// The escape hatch for the parts of JQL this module does not model — `startOfDay(-1d)`, `EMPTY`, an operator that
/// arrived after this was written. Whatever is handed to it is the caller's to make safe: a value that came from
/// outside the program does not belong here.
pub fn raw(fragment: impl Into<String>) -> Value {
    Value(fragment.into())
}

impl Field {
    /// `field = value`.
    #[must_use]
    pub fn eq(self, value: impl Into<Value>) -> Clause {
        self.compare("=", value)
    }

    /// `field != value`.
    #[must_use]
    pub fn ne(self, value: impl Into<Value>) -> Clause {
        self.compare("!=", value)
    }

    /// `field > value`.
    #[must_use]
    pub fn gt(self, value: impl Into<Value>) -> Clause {
        self.compare(">", value)
    }

    /// `field >= value`.
    #[must_use]
    pub fn gte(self, value: impl Into<Value>) -> Clause {
        self.compare(">=", value)
    }

    /// `field < value`.
    #[must_use]
    pub fn lt(self, value: impl Into<Value>) -> Clause {
        self.compare("<", value)
    }

    /// `field <= value`.
    #[must_use]
    pub fn lte(self, value: impl Into<Value>) -> Clause {
        self.compare("<=", value)
    }

    /// `field ~ value`, the text match.
    #[must_use]
    pub fn contains(self, value: impl Into<Value>) -> Clause {
        self.compare("~", value)
    }

    /// `field !~ value`.
    #[must_use]
    pub fn not_contains(self, value: impl Into<Value>) -> Clause {
        self.compare("!~", value)
    }

    /// `field IN (…)`.
    #[must_use]
    pub fn is_in(self, values: impl IntoIterator<Item = impl Into<Value>>) -> Clause {
        self.membership("IN", values)
    }

    /// `field NOT IN (…)`.
    #[must_use]
    pub fn not_in(self, values: impl IntoIterator<Item = impl Into<Value>>) -> Clause {
        self.membership("NOT IN", values)
    }

    /// `field IS EMPTY`.
    #[must_use]
    pub fn is_empty(self) -> Clause {
        Clause(Node::Comparison(format!("{} IS EMPTY", self.rendered())))
    }

    /// `field IS NOT EMPTY`.
    #[must_use]
    pub fn is_not_empty(self) -> Clause {
        Clause(Node::Comparison(format!("{} IS NOT EMPTY", self.rendered())))
    }

    fn compare(self, operator: &str, value: impl Into<Value>) -> Clause {
        Clause(Node::Comparison(format!("{} {operator} {}", self.rendered(), value.into().0)))
    }

    fn membership(self, operator: &str, values: impl IntoIterator<Item = impl Into<Value>>) -> Clause {
        let values = values.into_iter().map(|value| value.into().0).collect::<Vec<_>>().join(", ");

        Clause(Node::Comparison(format!("{} {operator} ({values})", self.rendered())))
    }

    fn rendered(&self) -> String {
        let plain = !self.0.is_empty()
            && self.0.starts_with(|character: char| character.is_ascii_alphabetic())
            && self.0.chars().all(|character| character.is_ascii_alphanumeric() || character == '_');

        // A custom field is written `cf[10001]`, brackets and all, and quoting it would make it a name to look up
        // rather than the field it addresses.
        if plain || self.0.starts_with("cf[") {
            return self.0.clone();
        }

        quote(&self.0)
    }
}

impl Clause {
    /// Both this and the other.
    #[must_use]
    pub fn and(self, other: Clause) -> Clause {
        match self.0 {
            Node::And(mut clauses) => {
                clauses.push(other.0);

                Clause(Node::And(clauses))
            }
            node => Clause(Node::And(vec![node, other.0])),
        }
    }

    /// Either this or the other.
    #[must_use]
    pub fn or(self, other: Clause) -> Clause {
        match self.0 {
            Node::Or(mut clauses) => {
                clauses.push(other.0);

                Clause(Node::Or(clauses))
            }
            node => Clause(Node::Or(vec![node, other.0])),
        }
    }

    /// Everything this does not match.
    #[must_use]
    pub fn negate(self) -> Clause {
        Clause(Node::Not(Box::new(self.0)))
    }

    /// Orders the results by a field, ascending.
    #[must_use]
    pub fn order_by(self, name: impl Into<String>) -> Query {
        Query { clause: self, order: Vec::new() }.order_by(name)
    }

    /// Orders the results by a field, descending.
    #[must_use]
    pub fn order_by_desc(self, name: impl Into<String>) -> Query {
        Query { clause: self, order: Vec::new() }.order_by_desc(name)
    }
}

impl Query {
    /// Adds a field to order by, ascending. Jira takes at most seven.
    #[must_use]
    pub fn order_by(mut self, name: impl Into<String>) -> Query {
        self.order.push((field(name), Direction::Ascending));

        self
    }

    /// Adds a field to order by, descending. Jira takes at most seven.
    #[must_use]
    pub fn order_by_desc(mut self, name: impl Into<String>) -> Query {
        self.order.push((field(name), Direction::Descending));

        self
    }
}

impl Node {
    fn rendered(&self) -> String {
        match self {
            Node::Comparison(text) => text.clone(),
            Node::And(clauses) => Node::joined(clauses, " AND "),
            Node::Or(clauses) => Node::joined(clauses, " OR "),
            Node::Not(clause) => format!("NOT {}", clause.grouped()),
        }
    }

    fn joined(clauses: &[Node], operator: &str) -> String {
        clauses.iter().map(Node::grouped).collect::<Vec<_>>().join(operator)
    }

    /// Renders a clause with the brackets its place in the tree calls for.
    ///
    /// `AND` binds tighter than `OR` in JQL, so a tree read back from an unbracketed rendering is not always the tree
    /// that was written. Bracketing every composite is a spelling of the same query that cannot be misread.
    fn grouped(&self) -> String {
        match self {
            Node::Comparison(text) => text.clone(),
            node => format!("({})", node.rendered()),
        }
    }
}

/// Wraps a value in quotation marks, with what JQL needs escaped inside them escaped.
fn quote(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);

    quoted.push('"');

    for character in value.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            _ => quoted.push(character),
        }
    }

    quoted.push('"');

    quoted
}

impl Display for Clause {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0.rendered())
    }
}

impl Display for Query {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.clause)?;

        if self.order.is_empty() {
            return Ok(());
        }

        let order = self
            .order
            .iter()
            .map(|(name, direction)| match direction {
                Direction::Ascending => format!("{} ASC", name.rendered()),
                Direction::Descending => format!("{} DESC", name.rendered()),
            })
            .collect::<Vec<_>>()
            .join(", ");

        write!(formatter, " ORDER BY {order}")
    }
}

impl From<Clause> for String {
    fn from(clause: Clause) -> String {
        clause.to_string()
    }
}

impl From<Query> for String {
    fn from(query: Query) -> String {
        query.to_string()
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Value {
        Value(quote(value))
    }
}

impl From<&String> for Value {
    fn from(value: &String) -> Value {
        Value(quote(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value(quote(&value))
    }
}

macro_rules! value_from_number {
    ($($number:ty),*) => {
        $(
            impl From<$number> for Value {
                fn from(value: $number) -> Value {
                    Value(value.to_string())
                }
            }
        )*
    };
}

value_from_number!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_value_cannot_end_the_literal_it_sits_in() {
        let clause = field("summary").contains(r#"say "hello" \ goodbye"#);

        assert_eq!(clause.to_string(), r#"summary ~ "say \"hello\" \\ goodbye""#);
    }

    #[test]
    fn a_value_cannot_smuggle_a_clause_of_its_own() {
        let clause = field("project").eq(r#"PROJ" OR project = "SECRET"#);

        assert_eq!(clause.to_string(), r#"project = "PROJ\" OR project = \"SECRET""#);
    }

    #[test]
    fn a_number_is_written_as_a_number() {
        assert_eq!(field("Story Points").gte(5).to_string(), r#""Story Points" >= 5"#);
    }

    #[test]
    fn a_field_is_quoted_only_when_it_has_to_be() {
        assert_eq!(field("project").is_empty().to_string(), "project IS EMPTY");
        assert_eq!(field("cf[10001]").is_not_empty().to_string(), "cf[10001] IS NOT EMPTY");
        assert_eq!(field("Story Points").is_empty().to_string(), r#""Story Points" IS EMPTY"#);
    }

    #[test]
    fn a_chain_of_one_operator_stays_flat() {
        let clause = field("a").eq("1").and(field("b").eq("2")).and(field("c").eq("3"));

        assert_eq!(clause.to_string(), r#"a = "1" AND b = "2" AND c = "3""#);
    }

    #[test]
    fn a_mixed_chain_says_what_it_means() {
        let clause = field("a").eq("1").and(field("b").eq("2").or(field("c").eq("3")));

        assert_eq!(clause.to_string(), r#"a = "1" AND (b = "2" OR c = "3")"#);
    }

    #[test]
    fn a_negation_covers_what_it_was_given() {
        let clause = field("a").eq("1").and(field("b").eq("2")).negate();

        assert_eq!(clause.to_string(), r#"NOT (a = "1" AND b = "2")"#);
    }

    #[test]
    fn a_list_is_a_list_of_values() {
        let clause = field("status").not_in(["Done", "In Progress"]);

        assert_eq!(clause.to_string(), r#"status NOT IN ("Done", "In Progress")"#);
        assert_eq!(field("id").is_in([1, 2, 3]).to_string(), "id IN (1, 2, 3)");
    }

    #[test]
    fn a_function_is_not_quoted_but_its_arguments_are() {
        assert_eq!(field("assignee").eq(func("currentUser")).to_string(), "assignee = currentUser()");
        assert_eq!(
            field("assignee").is_in([func_with("membersOf", ["jira-users"])]).to_string(),
            r#"assignee IN (membersOf("jira-users"))"#,
        );
    }

    #[test]
    fn a_raw_fragment_is_left_alone() {
        assert_eq!(field("created").gte(raw("startOfDay(-1d)")).to_string(), "created >= startOfDay(-1d)");
    }

    #[test]
    fn the_order_comes_last_and_keeps_its_fields_in_order() {
        let query = field("project").eq("PROJ").order_by_desc("created").order_by("key");

        assert_eq!(query.to_string(), r#"project = "PROJ" ORDER BY created DESC, key ASC"#);
    }

    #[test]
    fn a_query_reaches_an_operation_as_the_string_it_renders_to() {
        let query = field("project").eq("PROJ").order_by_desc("created");

        assert_eq!(String::from(query), r#"project = "PROJ" ORDER BY created DESC"#);
        assert_eq!(String::from(field("project").eq("PROJ")), r#"project = "PROJ""#);
    }
}
