use crate::filters::{FilterCondition, FilterConditionError, Literal};

impl FilterCondition {
    /// Evaluate a value against `FilterCondition` variant returning `Ok(true)` if value met filter condition and `Ok(false) if not`
    ///
    /// Returns `Err(FilterConditionError)` if runtime error occurs related to value types
    pub fn eval(&self, value: Option<&Literal>) -> Result<bool, FilterConditionError> {
        match (self, value) {
            (Self::Equals(other), Some(val)) => Ok(val == other),
            (Self::NotEquals(other), Some(val)) => Ok(val != other),
            (Self::Greater(Literal::Int(other)), Some(val)) => match val {
                Literal::Int(c) => Ok(c > other),
                e => Err(FilterConditionError::ExpectedPartiallyOrderableValue(
                    e.clone(),
                )),
            },
            (Self::GreaterOrEquals(Literal::Int(other)), Some(val)) => match val {
                Literal::Int(c) => Ok(c >= other),
                e => Err(FilterConditionError::ExpectedPartiallyOrderableValue(
                    e.clone(),
                )),
            },
            (Self::Lesser(Literal::Int(other)), Some(val)) => match val {
                Literal::Int(c) => Ok(c < other),
                e => Err(FilterConditionError::ExpectedPartiallyOrderableValue(
                    e.clone(),
                )),
            },
            (Self::LesserOrEquals(Literal::Int(other)), Some(val)) => match val {
                Literal::Int(c) => Ok(c <= other),
                e => Err(FilterConditionError::ExpectedPartiallyOrderableValue(
                    e.clone(),
                )),
            },
            (Self::In(array), Some(val)) => match val {
                Literal::Array(_) => Ok(false),
                v => Ok(array.contains(v)),
            },
            (Self::Contains(Literal::Str(substring)), Some(Literal::Str(val))) => {
                Ok(val.contains(substring))
            }
            (Self::StartsWith(prefix), Some(Literal::Str(val))) => Ok(val.starts_with(prefix)),
            (Self::EndsWith(suffix), Some(Literal::Str(val))) => Ok(val.ends_with(suffix)),
            (Self::Between { min, max }, Some(val)) => match val {
                Literal::Int(c) => Ok(min <= c && c <= max),
                e => Err(FilterConditionError::ExpectedPartiallyOrderableValue(
                    e.clone(),
                )),
            },
            (Self::IsNull, _) => Ok(value.is_none()),
            (Self::IsNotNull, _) => Ok(value.is_some()),
            (_, _) => Ok(false),
        }
    }
}

#[cfg(test)]
mod filter_condition_test_suite {
    use crate::filters::{FilterCondition, FilterConditionError, Literal};
    use rstest::rstest;

    #[rstest]
    #[case::eq_1(Literal::Int(5), FilterCondition::Equals(Literal::Int(5)))]
    #[case::eq_2(
        Literal::Str("test".to_string()),
        FilterCondition::Equals(Literal::Str("test".to_string()))
    )]
    #[case::ne_1(Literal::Int(5), FilterCondition::NotEquals(Literal::Int(4)))]
    #[case::gt_1(Literal::Int(5), FilterCondition::Greater(Literal::Int(4)))]
    #[case::ge_1(Literal::Int(5), FilterCondition::GreaterOrEquals(Literal::Int(4)))]
    #[case::ge_2(Literal::Int(5), FilterCondition::GreaterOrEquals(Literal::Int(5)))]
    #[case::lt_1(Literal::Int(5), FilterCondition::Lesser(Literal::Int(6)))]
    #[case::le_1(Literal::Int(5), FilterCondition::LesserOrEquals(Literal::Int(6)))]
    #[case::le_2(Literal::Int(5), FilterCondition::LesserOrEquals(Literal::Int(5)))]
    #[case::in_1(Literal::Int(5), FilterCondition::In(vec![Literal::Int(4), Literal::Int(5)]))]
    #[case::contains_1(
        Literal::Str("test".to_string()),
        FilterCondition::Contains(Literal::Str("es".to_string()))
    )]
    #[case::contains_2(
        Literal::Str("test".to_string()),
        FilterCondition::Contains(Literal::Str("test".to_string()))
    )]
    #[case::starts_with_1(
        Literal::Str("test".to_string()),
        FilterCondition::StartsWith("te".to_string())
    )]
    #[case::ends_with_1(
        Literal::Str("test".to_string()),
        FilterCondition::EndsWith("st".to_string())
    )]
    #[case::between_1(
        Literal::Int(5),
        FilterCondition::Between { min: 3, max: 7 },
    )]
    #[case::between_2(
        Literal::Int(7),
        FilterCondition::Between { min: 3, max: 7 },
    )]
    #[case::between_3(
        Literal::Int(3),
        FilterCondition::Between { min: 3, max: 7 },
    )]
    #[case::between_4(
        Literal::Int(4),
        FilterCondition::Between { min: 4, max: 4 },
    )]
    fn predicate_returns_true(#[case] value: Literal, #[case] sut: FilterCondition) {
        let result = sut.eval(Some(value).as_ref());

        assert!(result.is_ok_and(|x| x));
    }

    #[rstest]
    #[case::eq_1(Literal::Int(4), FilterCondition::Equals(Literal::Int(5)))]
    #[case::eq_2(
        Literal::Str("test_not".to_string()),
        FilterCondition::Equals(Literal::Str("test".to_string()))
    )]
    #[case::eq_3_different_literals(
        Literal::Int(5),
        FilterCondition::Equals(Literal::Str("test".to_string()))
    )]
    #[case::ne_1(Literal::Int(5), FilterCondition::NotEquals(Literal::Int(5)))]
    #[case::gt_1(Literal::Int(4), FilterCondition::Greater(Literal::Int(5)))]
    #[case::gt_2(Literal::Int(4), FilterCondition::Greater(Literal::Int(8)))]
    #[case::ge_1(Literal::Int(3), FilterCondition::GreaterOrEquals(Literal::Int(5)))]
    #[case::lt_1(Literal::Int(7), FilterCondition::Lesser(Literal::Int(6)))]
    #[case::lt_2(Literal::Int(7), FilterCondition::Lesser(Literal::Int(7)))]
    #[case::le_1(Literal::Int(10), FilterCondition::LesserOrEquals(Literal::Int(6)))]
    #[case::in_1(Literal::Int(7), FilterCondition::In(vec![Literal::Int(4), Literal::Int(5)]))]
    #[case::contains_1(
        Literal::Str("test".to_string()),
        FilterCondition::Contains(Literal::Str("esr".to_string()))
    )]
    #[case::contains_2(
        Literal::Str("test".to_string()),
        FilterCondition::Contains(Literal::Str("tester".to_string()))
    )]
    #[case::starts_with_1(
        Literal::Str("test".to_string()),
        FilterCondition::StartsWith("est".to_string())
    )]
    #[case::ends_with_1(
        Literal::Str("test".to_string()),
        FilterCondition::EndsWith("tes".to_string())
    )]
    #[case::between_1(
        Literal::Int(2),
        FilterCondition::Between { min: 3, max: 7 },
    )]
    #[case::between_2(
        Literal::Int(8),
        FilterCondition::Between { min: 3, max: 7 },
    )]
    fn predicate_returns_false(#[case] value: Literal, #[case] sut: FilterCondition) {
        let result = sut.eval(Some(value).as_ref());

        assert!(result.is_ok_and(|x| !x));
    }

    #[rstest]
    #[case::gt_with_string(
        Literal::Str("test".to_string()),
        FilterCondition::Greater(Literal::Int(5)),
        FilterConditionError::ExpectedPartiallyOrderableValue(
            Literal::Str("test".to_string())
        ),
    )]
    #[case::ge_with_string(
        Literal::Str("test".to_string()),
        FilterCondition::GreaterOrEquals(Literal::Int(5)),
        FilterConditionError::ExpectedPartiallyOrderableValue(
            Literal::Str("test".to_string())
        ),
    )]
    #[case::lt_with_string(
        Literal::Str("test".to_string()),
        FilterCondition::Lesser(Literal::Int(5)),
        FilterConditionError::ExpectedPartiallyOrderableValue(
            Literal::Str("test".to_string())
        ),
    )]
    #[case::le_with_string(
        Literal::Str("test".to_string()),
        FilterCondition::LesserOrEquals(Literal::Int(5)),
        FilterConditionError::ExpectedPartiallyOrderableValue(
            Literal::Str("test".to_string())
        ),
    )]
    #[case::between_with_string(
        Literal::Str("test".to_string()),
        FilterCondition::Between { min: 3, max: 7 },
        FilterConditionError::ExpectedPartiallyOrderableValue(
            Literal::Str("test".to_string())
        )
    )]
    fn predicate_returns_error(
        #[case] value: Literal,
        #[case] sut: FilterCondition,
        #[case] expected_error: FilterConditionError,
    ) {
        let result = sut.eval(Some(value).as_ref());

        assert_eq!(result, Result::Err(expected_error));
    }
}
