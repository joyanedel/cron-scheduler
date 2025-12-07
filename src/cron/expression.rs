use super::expression_component::CronExpressionComponent;
use std::str::FromStr;

/// Cron expression which is formed by 5 components
/// - Minute
/// - Hour
/// - Day
/// - Month
/// - Weekday
///
/// Expressions accepted
/// - Single value: a
/// - Range: a-b -> From a to b
/// - List: a,b,c,d -> At a, at b, at c and at d
/// - Step: a/b -> From a take steps of b size
/// - All: * -> All possible values
///
/// Example: 0 0 * * * means every day at time 00:00
#[derive(Debug, PartialEq)]
pub struct CronExpression {
    minute: CronExpressionComponent,
    hour: CronExpressionComponent,
    day: CronExpressionComponent,
    month: CronExpressionComponent,
    weekday: CronExpressionComponent,
}

#[derive(Debug, PartialEq)]
pub enum CronExpressionError {
    Malformed,
    InvalidComponent,
}

impl FromStr for CronExpression {
    type Err = CronExpressionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [minute, hour, day, month, weekday]: [CronExpressionComponent; 5] = s
            .split_whitespace()
            .map(CronExpressionComponent::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| CronExpressionError::InvalidComponent)?
            .try_into()
            .map_err(|_| CronExpressionError::Malformed)?;
        Ok(Self {
            minute,
            hour,
            day,
            month,
            weekday,
        })
    }
}

#[cfg(test)]
mod cron_expression_tests {
    use std::str::FromStr;

    use crate::cron::{expression::CronExpression, expression_component::CronExpressionComponent};

    #[test]
    fn initialize_cron_expression_with_all_possible_values_succeed() {
        let input = "* * * * *";
        let result = CronExpression::from_str(input);
        assert!(result.is_ok_and(|x| x
            == CronExpression {
                minute: CronExpressionComponent::All,
                hour: CronExpressionComponent::All,
                day: CronExpressionComponent::All,
                month: CronExpressionComponent::All,
                weekday: CronExpressionComponent::All
            }))
    }

    #[test]
    fn regular_cron_expression_parsed_correctly() {
        let input = "0,30 8-20 * * ?";
        let result = CronExpression::from_str(input);
        assert!(result.is_ok_and(|x| x
            == CronExpression {
                minute: CronExpressionComponent::List(vec![
                    CronExpressionComponent::Value(0),
                    CronExpressionComponent::Value(30)
                ]),
                hour: CronExpressionComponent::Range(8, 20),
                day: CronExpressionComponent::All,
                month: CronExpressionComponent::All,
                weekday: CronExpressionComponent::Ignore
            }))
    }
}
