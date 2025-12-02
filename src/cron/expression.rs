use std::str::FromStr;
use super::expression_component::CronExpressionComponent;

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
struct CronExpression {
    minute: CronExpressionComponent,
    hour: CronExpressionComponent,
    day: CronExpressionComponent,
    month: CronExpressionComponent,
    weekday: CronExpressionComponent,
}

enum CronExpressionError {
    Malformed,
    InvalidComponent,
}

impl FromStr for CronExpression {
    type Err = CronExpressionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod cron_expression_tests {
    #[test]
    fn initialize_cron_expression_with_all_possible_values_succeed() {
        unimplemented!()
    }
}
