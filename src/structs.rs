use std::num::Saturating;

use serde::Deserialize;
use thiserror::Error;

/// Filter possible variants
///
/// Let developer define highest filter precedent predicates `and`, `or` and `not`.
/// If none of them is desired, use `Predicate` variant by default
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum FilterNode<T> {
    /// Attribute "and" must be presented in data to be parsed as `And`
    #[serde(rename = "and")]
    And(Vec<FilterNode<T>>),

    /// Attribute "or" must be presebt in data to be parsed as `Or`
    #[serde(rename = "or")]
    Or(Vec<FilterNode<T>>),

    /// Attribute "not" must be present in data to be parsed as `Not`
    #[serde(rename = "not")]
    Not(Box<FilterNode<T>>),

    /// If none of above is present, predicate struct must match to be parsed as `Predicate`
    #[serde(untagged)]
    Predicate(Predicate<T>),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Predicate<T> {
    /// Field path to extract value to use in filter
    pub(crate) field: String,

    /// Condition to be used to filter field value
    #[serde(flatten)]
    pub(crate) condition: FilterCondition<T>,
}

/// `FilterCondition` is a base filter enumerator that list all allowed operations
/// within library.
///
/// Operations considerated are: `eq`, `gt`, `ge`, `lt`, `le` which requires :PartialOrd single values.
/// Operations `contains`, `starts_with`, `ends_with` requires value to be an string
/// Operation `in` requires value to be an array
/// Operation `between` requires value to be a 2-tuple with :PartialOrd values such as `(1, 2)`
/// Operation `is_null` requires value to be nullable
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "operator", content = "value")]
pub enum FilterCondition<T> {
    /// Requires value to be :PartialEq, such as integers and strings
    #[serde(rename = "eq")]
    Equals(T),

    /// Requires value to be :PartialOrd
    #[serde(rename = "gt")]
    Greater(T),

    /// Requires value to be :PartialOrd
    #[serde(rename = "ge")]
    GreaterOrEquals(T),

    /// Requires value to be :PartialOrd
    #[serde(rename = "lt")]
    Lesser(T),

    /// Requires value to be :PartialOrd
    #[serde(rename = "le")]
    LesserOrEquals(T),

    /// Requires value to be :PartialEq
    #[serde(rename = "in")]
    In(Vec<T>),

    /// Requires value to be String
    #[serde(rename = "contains")]
    Contains(String),

    /// Requires value to be String
    #[serde(rename = "starts_with")]
    StartsWith(String),

    /// Requires value to be String
    #[serde(rename = "ends_with")]
    EndsWith(String),

    /// Requires value to be String
    #[serde(rename = "between")]
    Between((T, T)),

    #[serde(rename = "is_null")]
    IsNull,

    #[serde(rename = "is_not_null")]
    IsNotNull,
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
