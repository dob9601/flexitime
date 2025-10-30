use chrono::NaiveTime;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::{tag_no_case, take_while_m_n},
    character::complete::{char, space0},
    combinator::{map_res, opt, peek, value},
    sequence::preceded,
};
use strum_macros::Display;

use crate::error::FlexitimeResult2;

#[derive(PartialEq, Debug, Clone)]
pub struct WallClockTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl WallClockTime {
    pub fn to_naive_time(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.hour.into(), self.minute.into(), self.second.into()).unwrap()
    }
}

#[derive(PartialEq, Debug, Display)]
pub enum WallClockTimeError {
    OutOfRangeHours,
    OutOfRangeMinutes,
    OutOfRangeSeconds,
}

impl std::error::Error for WallClockTimeError {}

impl WallClockTime {
    pub fn new(
        mut hour: u8,
        minute: u8,
        second: u8,
        period: Option<TimePeriod>,
    ) -> Result<Self, WallClockTimeError> {
        if let Some(TimePeriod::Pm) = period {
            hour += 12;

            if hour == 24 {
                hour = 0;
            }
        }

        if hour > 23 {
            return Err(WallClockTimeError::OutOfRangeHours);
        }
        if minute > 59 {
            return Err(WallClockTimeError::OutOfRangeMinutes);
        }
        if second > 59 {
            return Err(WallClockTimeError::OutOfRangeSeconds);
        }

        Ok(WallClockTime {
            hour,
            minute,
            second,
        })
    }
}

fn parse_u8(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse()
}

fn parse_hours(input: &str) -> FlexitimeResult2<&str, u8> {
    map_res(take_while_m_n(1, 2, |c: char| c.is_ascii_digit()), parse_u8).parse(input)
}

fn parse_optional_mins_or_secs(input: &str) -> FlexitimeResult2<&str, Option<u8>> {
    if let Ok((_, _)) = peek(char::<&str, nom::error::Error<&str>>(':')).parse(input) {
        let (input, seconds) = preceded(
            char(':'),
            map_res(take_while_m_n(2, 2, |c: char| c.is_ascii_digit()), parse_u8),
        )
        .parse(input)?;

        return Ok((input, Some(seconds)));
    }

    Ok((input, None))
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimePeriod {
    Am,
    Pm,
}

fn parse_am_pm_suffix(input: &str) -> FlexitimeResult2<&str, Option<TimePeriod>> {
    opt(preceded(
        space0,
        alt((
            value(TimePeriod::Am, tag_no_case("am")),
            value(TimePeriod::Pm, tag_no_case("pm")),
        )),
    ))
    .parse(input)
}

pub fn parse_wall_clock_time(input: &str) -> FlexitimeResult2<&str, WallClockTime> {
    map_res(
        (
            parse_hours,
            parse_optional_mins_or_secs,
            parse_optional_mins_or_secs,
            parse_am_pm_suffix,
        ),
        |(hours, minutes, seconds, period)| {
            WallClockTime::new(hours, minutes.unwrap_or(0), seconds.unwrap_or(0), period)
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {

    use crate::error::FlexitimeError2;

    use super::*;

    #[test]
    fn test_parse_hours_mins() {
        assert_eq!(
            parse_wall_clock_time("12:05"),
            Ok(("", WallClockTime::new(12, 5, 0, None).unwrap()))
        )
    }

    #[test]
    fn test_parse_hours_leading_zero() {
        assert_eq!(
            parse_wall_clock_time("07:05"),
            Ok(("", WallClockTime::new(7, 5, 0, None).unwrap()))
        )
    }

    #[test]
    fn test_parse_hours_single_digit() {
        assert_eq!(
            parse_wall_clock_time("7:05"),
            Ok(("", WallClockTime::new(7, 5, 0, None).unwrap()))
        )
    }

    #[test]
    fn test_parse_hours_mins_secs() {
        assert_eq!(
            parse_wall_clock_time("12:05:30"),
            Ok(("", WallClockTime::new(12, 5, 30, None).unwrap()))
        )
    }

    #[test]
    fn test_hours_out_of_range() {
        assert_eq!(
            parse_wall_clock_time("25:05:30"),
            Err(nom::Err::Error(FlexitimeError2::WallClockTime(
                WallClockTimeError::OutOfRangeHours
            )))
        )
    }

    #[test]
    fn test_mins_out_of_range() {
        assert_eq!(
            parse_wall_clock_time("23:65:30"),
            Err(nom::Err::Error(FlexitimeError2::WallClockTime(
                WallClockTimeError::OutOfRangeMinutes
            )))
        )
    }

    #[test]
    fn test_secs_out_of_range() {
        assert_eq!(
            parse_wall_clock_time("23:05:60 am"),
            Err(nom::Err::Error(FlexitimeError2::WallClockTime(
                WallClockTimeError::OutOfRangeSeconds
            )))
        )
    }

    #[test]
    fn test_parse_time_with_am() {
        assert_eq!(
            parse_wall_clock_time("12:05:30 am"),
            Ok((
                "",
                WallClockTime::new(12, 5, 30, Some(TimePeriod::Am)).unwrap()
            ))
        )
    }

    #[test]
    fn test_parse_time_with_pm() {
        assert_eq!(
            parse_wall_clock_time("8:05:30 pm"),
            Ok((
                "",
                WallClockTime::new(8, 5, 30, Some(TimePeriod::Pm)).unwrap()
            ))
        )
    }

    #[test]
    fn test_parse_time_with_pm_boundary() {
        assert_eq!(
            parse_wall_clock_time("12:05:30 pm"),
            Ok((
                "",
                WallClockTime::new(12, 5, 30, Some(TimePeriod::Pm)).unwrap()
            ))
        )
    }

    #[test]
    fn test_parse_time_with_mixed_case_period() {
        assert_eq!(
            parse_wall_clock_time("8:05:30 pM"),
            Ok((
                "",
                WallClockTime::new(8, 5, 30, Some(TimePeriod::Pm)).unwrap()
            ))
        )
    }
    #[test]
    fn test_parse_time_with_no_space_period() {
        assert_eq!(
            parse_wall_clock_time("3:00pm"),
            Ok((
                "",
                WallClockTime::new(3, 0, 0, Some(TimePeriod::Pm)).unwrap()
            ))
        )
    }

    #[test]
    fn test_parse_time_no_mins() {
        assert_eq!(
            parse_wall_clock_time("3pm"),
            Ok((
                "",
                WallClockTime::new(3, 0, 0, Some(TimePeriod::Pm)).unwrap()
            ))
        )
    }
}
