use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum CronExpressionComponent {
    Value(u8),
    Range(u8, u8),
    List(Vec<CronExpressionComponent>),
    Step(Box<CronExpressionComponent>, u8),
    All,
    Ignore,
}

#[derive(PartialEq, Debug)]
pub enum CronExpressionComponentError {
    InvalidValue,
}

impl FromStr for CronExpressionComponent {
    type Err = CronExpressionComponentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "*" {
            return Ok(Self::All);
        } else if s == "?" {
            return Ok(Self::Ignore);
        } else if s.contains("/") {
            let values = s.split_once("/");
            let (expr, step) = match values {
                Some((e, s)) => (e, s),
                _ => return Err(CronExpressionComponentError::InvalidValue),
            };
            let step =
                u8::from_str(step).map_err(|_| CronExpressionComponentError::InvalidValue)?;

            return Ok(Self::Step(Box::new(Self::from_str(expr)?), step));
        } else if s.contains(",") {
            let values = s.split(",");
            let results: Result<Vec<_>, _> =
                values.map(CronExpressionComponent::from_str).collect();

            return Ok(Self::List(results?));
        } else if s.contains("-") {
            let values = s.split_once("-");
            let (start, end) = match values {
                Some((s, e)) => (s, e),
                _ => return Err(CronExpressionComponentError::InvalidValue),
            };

            let start =
                u8::from_str(start).map_err(|_| CronExpressionComponentError::InvalidValue)?;
            let end = u8::from_str(end).map_err(|_| CronExpressionComponentError::InvalidValue)?;

            return Ok(Self::Range(start, end));
        } else {
            let single_value = u8::from_str(s);
            match single_value {
                Ok(v) => Ok(Self::Value(v)),
                _ => Err(CronExpressionComponentError::InvalidValue),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::cron::expression_component::{
        CronExpressionComponent, CronExpressionComponentError,
    };

    #[test]
    fn asterisk_is_parsed_as_all() {
        let input = "*";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x == CronExpressionComponent::All));
    }

    #[test]
    fn closing_question_is_parsed_as_ignore() {
        let input = "?";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x == CronExpressionComponent::Ignore));
    }

    #[test]
    fn integer_is_parsed_as_single_value() {
        let input = "54";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x == CronExpressionComponent::Value(54)));
    }

    #[test]
    fn non_valid_integer_raise_invalid_value_error() {
        let input = "0.3";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_err_and(|x| x == CronExpressionComponentError::InvalidValue));
    }

    #[test]
    fn range_is_parsed_as_range() {
        let input = "3-5";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x == CronExpressionComponent::Range(3, 5)));
    }

    #[test]
    fn two_single_value_comma_separated_values_is_parsed_as_list_of_single_values() {
        let input = "1,10";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x
            == CronExpressionComponent::List(vec![
                CronExpressionComponent::Value(1),
                CronExpressionComponent::Value(10)
            ])))
    }

    #[test]
    fn comma_separated_single_and_range_values_are_parsed_as_list_of_single_and_range_value() {
        let input = "2-5,10";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x
            == CronExpressionComponent::List(vec![
                CronExpressionComponent::Range(2, 5),
                CronExpressionComponent::Value(10)
            ])))
    }

    #[test]
    fn invalid_range_raises_invalid_value_error() {
        let input = "5-";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_err_and(|x| x == CronExpressionComponentError::InvalidValue));
    }

    #[test]
    fn step_single_value_is_parsed_as_step() {
        let input = "5/15";
        let result = CronExpressionComponent::from_str(input);
        assert!(
            result.is_ok_and(|x| x
                == CronExpressionComponent::Step(Box::new(CronExpressionComponent::Value(5)), 15))
        )
    }

    #[test]
    fn complex_but_valid_expression_is_parsed_correctly() {
        let input = "1,2,3-10/10";
        let result = CronExpressionComponent::from_str(input);
        assert!(result.is_ok_and(|x| x
            == CronExpressionComponent::Step(
                Box::new(CronExpressionComponent::List(vec![
                    CronExpressionComponent::Value(1),
                    CronExpressionComponent::Value(2),
                    CronExpressionComponent::Range(3, 10),
                ])),
                10
            )))
    }
}
