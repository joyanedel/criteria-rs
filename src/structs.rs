use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub enum FilterNode {
    And(Vec<FilterNode>),
    Or(Vec<FilterNode>),
    Not(Box<FilterNode>),
    Predicate(Predicate),
}

#[derive(Deserialize, Debug)]
pub struct Predicate {
    pub(crate) field: String,
    pub(crate) operator: Operator,
    pub(crate) value: String,
}

/// `Operator` is a base filter enumerator that list all allowed operations
/// within library.
///
/// Operations considerated are: `eq`, `gt`, `ge`, `lt`, `le` which requires :PartialOrd single values.
/// Operations `contains`, `starts_with`, `ends_with` requires value to be an string
/// Operation `in` requires value to be an array
/// Operation `between` requires value to be a 2-tuple with :PartialOrd values such as `(1, 2)`
/// Operation `is_null` requires value to be nullable
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum Operator {
    /// Requires value to be :PartialEq, such as integers and strings
    #[serde(rename = "eq")]
    Equals,

    /// Requires value to be :PartialOrd
    #[serde(rename = "gt")]
    Greater,

    /// Requires value to be :PartialOrd
    #[serde(rename = "ge")]
    GreaterOrEquals,

    /// Requires value to be :PartialOrd
    #[serde(rename = "lt")]
    Lesser,

    /// Requires value to be :PartialOrd
    #[serde(rename = "le")]
    LesserOrEquals,

    /// Requires value to be :PartialEq
    #[serde(rename = "in")]
    In,

    /// Requires value to be :PartialEq
    #[serde(rename = "contains")]
    Contains,

    /// Requires value to be :PartialEq
    #[serde(rename = "starts_with")]
    StartsWith,

    /// Requires value to be :PartialEq
    #[serde(rename = "ends_with")]
    EndsWith,

    /// Requires value to be :PartialOrd
    #[serde(rename = "between")]
    Between,

    /// Requires value to be nullable
    #[serde(rename = "is_null")]
    IsNull,
}

#[derive(Error, Debug)]
pub enum PredicateError {
    #[error("value must be of type `string`, got `{0}`")]
    ExpectedString(String),

    #[error("value must be of type `array`, got `{0}`")]
    ExpectedArray(String),

    #[error("value must be orderable, got `{0}` which does not implement :PartialOrd")]
    ExpectedPartiallyOrderableValue(String),

    #[error("value must be of type `tuple` and :PartialOrd, got `{0}`")]
    ExpectedTuple(String),
}
