use chrono::{DateTime, Utc};

use crate::cron::expression::{CronExpression, CronExpressionError};

fn expression_matches_datetime(
    expr: CronExpression,
    datetime: DateTime<Utc>,
) -> Result<bool, CronExpressionError> {
    unimplemented!()
}

#[cfg(test)]
mod test_expression_matches_datetimetests {
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
