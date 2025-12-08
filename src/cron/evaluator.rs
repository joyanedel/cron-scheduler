use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::cron::{
    expression::{CronExpression, CronExpressionError},
    expression_component::CronExpressionComponent,
};

fn expression_matches_datetime(
    expr: CronExpression,
    datetime: DateTime<Utc>,
) -> Result<bool, CronExpressionError> {
    if !expression_component_matches_number(expr.minute, datetime.minute()) {
        return Ok(false);
    } else if !expression_component_matches_number(expr.hour, datetime.hour()) {
        return Ok(false);
    }

    if !expression_component_matches_number(expr.day, datetime.day())
        && !expression_component_matches_number(
            expr.weekday,
            datetime.weekday().number_from_monday(),
        )
    {
        return Ok(false);
    } else if !expression_component_matches_number(expr.month, datetime.month()) {
        return Ok(false);
    }

    return Ok(true);
}

fn expression_component_matches_number(
    expr_component: CronExpressionComponent,
    date_component: u32,
) -> bool {
    match expr_component {
        CronExpressionComponent::All => true,
        CronExpressionComponent::Value(n) => n as u32 == date_component,
        CronExpressionComponent::Ignore => true,
        CronExpressionComponent::Range(a, b) => {
            a as u32 <= date_component && date_component <= b as u32
        }
        CronExpressionComponent::Step(ce, step) => match ce.as_ref() {
            CronExpressionComponent::Range(a, b) if a <= b => {
                if *a as u32 == date_component {
                    true
                } else {
                    expression_component_matches_number(
                        CronExpressionComponent::Step(
                            Box::new(CronExpressionComponent::Range(a + step, *b)),
                            step,
                        ),
                        date_component,
                    )
                }
            }
            // use 31 as the maximum number to be used in a cron expression
            // minutes and hours goes from 0 to 59
            // days from 1 to 31
            // months from 1 to 12
            // weekdays from 1 to 7
            CronExpressionComponent::Value(n) if *n <= 59 => {
                if *n as u32 == date_component {
                    true
                } else {
                    expression_component_matches_number(
                        CronExpressionComponent::Step(
                            Box::new(CronExpressionComponent::Value(n + step)),
                            step,
                        ),
                        date_component,
                    )
                }
            }
            _ => false,
        },
        CronExpressionComponent::List(v) => v
            .iter()
            .map(|e| expression_component_matches_number(e.clone(), date_component))
            .any(|v| v),
    }
}

#[cfg(test)]
mod test_expression_matches_datetime {
    use std::str::FromStr;

    use crate::cron::{evaluator::expression_matches_datetime, expression::CronExpression};

    #[test]
    fn all_expression_matches_any_date() {
        let cron_expression = CronExpression::from_str("* * * * *").unwrap();
        let date = chrono::NaiveDateTime::parse_from_str("2025-10-10 00:00:00", "%F %T")
            .unwrap()
            .and_utc();
        let result = expression_matches_datetime(cron_expression, date);
        assert_eq!(result, Ok(true))
    }

    #[test]
    fn date_with_non_matching_expression_returns_false() {
        let cron_expression = CronExpression::from_str("30 12 31 12 ?").unwrap();
        let date = chrono::NaiveDateTime::parse_from_str("2025-12-31 12:31:00", "%F %T")
            .unwrap()
            .and_utc();
        let result = expression_matches_datetime(cron_expression, date);
        assert_eq!(result, Ok(false))
    }

    #[test]
    fn date_with_matching_expression_returns_true() {
        let cron_expression = CronExpression::from_str("30 12 31 12 ?").unwrap();
        let date = chrono::NaiveDateTime::parse_from_str("2025-12-31 12:31:00", "%F %T")
            .unwrap()
            .and_utc();
        let result = expression_matches_datetime(cron_expression, date);
        assert_eq!(result, Ok(false))
    }

    #[test]
    fn date_with_range_matching_expression_returns_true() {
        let cron_expression = CronExpression::from_str("* 12-20 1 1 ?").unwrap();
        let date = chrono::NaiveDateTime::parse_from_str("2025-01-01 15:00:00", "%F %T")
            .unwrap()
            .and_utc();
        let result = expression_matches_datetime(cron_expression, date);
        assert_eq!(result, Ok(true))
    }

    #[test]
    fn date_outside_range_matching_expression_returns_false() {
        let cron_expression = CronExpression::from_str("* 12-20 1 1 ?").unwrap();
        let date = chrono::NaiveDateTime::parse_from_str("2025-01-01 06:30:00", "%F %T")
            .unwrap()
            .and_utc();
        let result = expression_matches_datetime(cron_expression, date);
        assert_eq!(result, Ok(false))
    }
}

#[cfg(test)]
mod test_expression_component_matches_nunber {
    use crate::cron::{
        evaluator::expression_component_matches_number,
        expression_component::CronExpressionComponent,
    };

    #[test]
    fn all_component_matches_any_number() {
        let expr_component = CronExpressionComponent::All;
        let date_component = 13;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn single_value_component_matches_number() {
        let expr_component = CronExpressionComponent::Value(15);
        let date_component = 15;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn single_value_component_doesnt_match_number() {
        let expr_component = CronExpressionComponent::Value(15);
        let date_component = 14;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(!result)
    }

    #[test]
    fn list_of_single_values_component_matches_number() {
        let expr_component = CronExpressionComponent::List(vec![
            CronExpressionComponent::Value(13),
            CronExpressionComponent::Value(15),
        ]);
        let date_component = 15;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn range_value_component_matches_number() {
        let expr_component = CronExpressionComponent::Range(10, 20);
        let date_component = 15;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn range_value_component_doesn_match_number_out_of_range() {
        let expr_component = CronExpressionComponent::Range(10, 20);
        let date_component = 21;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(!result)
    }

    #[test]
    fn step_value_component_matches_number() {
        let expr_component =
            CronExpressionComponent::Step(Box::new(CronExpressionComponent::Value(2)), 5);
        let date_component = 12;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn step_value_component_doesnt_match_number_unreachable() {
        let expr_component =
            CronExpressionComponent::Step(Box::new(CronExpressionComponent::Value(2)), 5);
        let date_component = 13;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(!result)
    }

    #[test]
    fn step_ranged_component_matches_number() {
        let expr_component =
            CronExpressionComponent::Step(Box::new(CronExpressionComponent::Range(5, 10)), 2);
        let date_component = 9;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(result)
    }

    #[test]
    fn step_ranged_component_doesn_match_number_out_of_range() {
        let expr_component =
            CronExpressionComponent::Step(Box::new(CronExpressionComponent::Range(5, 10)), 2);
        let date_component = 8;
        let result = expression_component_matches_number(expr_component, date_component);
        assert!(!result)
    }
}
