use nom::{IResult, Parser, character::complete::space1, multi::separated_list1};
use suffix::Suffix;
use time::RelativeTime;
use units::RelativeUnit;

use super::time::ParsedTime;

mod suffix;
pub mod time;
mod units;

pub fn parse_relative_time(input: &str) -> IResult<&str, ParsedTime> {
    let (input, (units, suffix)) = (
        separated_list1(space1, units::parse_unit),
        suffix::parse_suffix,
    )
        .parse(input)?;

    let mut time = RelativeTime::default();
    for unit in units {
        match unit.unit {
            RelativeUnit::Seconds => time.seconds = Some(unit.amount),
            RelativeUnit::Minutes => time.minutes = Some(unit.amount),
            RelativeUnit::Hours => time.hours = Some(unit.amount),
            RelativeUnit::Days => time.days = Some(unit.amount),
            RelativeUnit::Weeks => time.weeks = Some(unit.amount),
            RelativeUnit::Months => time.months = Some(unit.amount),
            RelativeUnit::Years => time.years = Some(unit.amount),
        }
    }

    if let Some(suffix) = suffix
        && suffix == Suffix::Ago
    {
        time.negative = true;
    }

    Ok((input, ParsedTime::Relative(time)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_relative_timestring() {
        assert_eq!(
            parse_relative_time("2d 1h"),
            Ok((
                "",
                ParsedTime::Relative(RelativeTime::new().days(2).hours(1))
            ))
        );
    }

    #[test]
    fn test_parse_negative_relative_timestring() {
        assert_eq!(
            parse_relative_time("2d 1h ago"),
            Ok((
                "",
                ParsedTime::Relative(RelativeTime::new().days(2).hours(1).ago())
            ))
        );
    }
}
