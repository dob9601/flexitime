use nom::{Parser, branch::alt, combinator::map};
use time::ParsedTime;

use crate::error::FlexitimeResult;

pub mod absolute;
pub mod relative;
mod time;

pub fn parse_timestring(input: &str) -> FlexitimeResult<&str, ParsedTime> {
    alt((
        map(relative::parse_relative_time, |t| ParsedTime::Relative(t)),
        map(absolute::parse_absolute_time, |t| ParsedTime::Absolute(t)),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::absolute::{
        AbsoluteTimeBuilder, DayOffset, FlexiDate, TimePeriod, WallClockTime,
    };

    use super::*;

    #[test]
    fn test_parse_time() {
        assert_eq!(
            parse_timestring("3pm tomorrow").unwrap(),
            (
                "",
                ParsedTime::Absolute(
                    AbsoluteTimeBuilder::new()
                        .date(FlexiDate::DayOffset(DayOffset::Fixed(1)))
                        .time(WallClockTime::new(3, 0, 0, Some(TimePeriod::Pm)).unwrap())
                        .build()
                        .unwrap()
                )
            )
        )
    }
}
