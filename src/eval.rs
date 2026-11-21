use crate::filters::{FilterCondition, FilterConditionError, Literal};

impl FilterCondition {
    fn eval(&self, value: Option<&Literal>) -> Result<bool, FilterConditionError> {
        match (self, value) {
            (Self::Equals(other), Some(val)) => Ok(val == other),
            (Self::NotEquals(other), Some(val)) => Ok(val != other),
            (Self::Greater(Literal::Int(other)), Some(Literal::Int(val))) => Ok(val > other),
            (Self::GreaterOrEquals(Literal::Int(other)), Some(Literal::Int(val))) => {
                Ok(val >= other)
            }
            (Self::Lesser(Literal::Int(other)), Some(Literal::Int(val))) => Ok(val < other),
            (Self::LesserOrEquals(Literal::Int(other)), Some(Literal::Int(val))) => {
                Ok(val <= other)
            }
            (Self::In(array), Some(val)) => match val {
                Literal::Array(_) => Ok(false),
                v => Ok(array.contains(v)),
            },
            (Self::Contains(Literal::Str(substring)), Some(Literal::Str(val))) => {
                Ok(val.contains(substring))
            }
            (Self::StartsWith(prefix), Some(Literal::Str(val))) => Ok(val.starts_with(prefix)),
            (Self::EndsWith(suffix), Some(Literal::Str(val))) => Ok(val.ends_with(suffix)),
            (Self::Between { min, max }, Some(Literal::Int(val))) => Ok(min <= val && val <= max),
            (Self::IsNull, _) => Ok(value.is_none()),
            (Self::IsNotNull, _) => Ok(value.is_some()),
            (_, _) => Ok(false),
        }
    }
}

#[cfg(test)]
mod filter_condition_test_suite {
    use crate::filters::{FilterCondition, Literal};
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
}
