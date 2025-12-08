use chrono::{DateTime, Utc};

use crate::cron::{
    expression::{CronExpression, CronExpressionError},
    expression_component::CronExpressionComponent,
};

fn expression_matches_datetime(
    expr: CronExpression,
    datetime: DateTime<Utc>,
) -> Result<bool, CronExpressionError> {
    unimplemented!()
}

fn expression_component_matches_number(
    expr_component: CronExpressionComponent,
    date_component: u32,
) -> bool {
    unimplemented!()
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
        assert!(result)
    }
}
