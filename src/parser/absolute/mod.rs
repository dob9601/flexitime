use chrono::{NaiveTime, Utc};
use nom::{
    IResult, Parser, branch::alt, character::complete::space1, combinator::map,
    sequence::separated_pair,
};
use time::parse_wall_clock_time;
use timezone::parse_timezone;

use super::time::ParsedTime;

mod time;
mod timezone;

pub fn parse_absolute_time(input: &str) -> IResult<&str, ParsedTime> {
    map(
        alt((separated_pair(
            parse_wall_clock_time,
            space1,
            parse_timezone,
        ),)),
        |(raw_time, tz)| {
            ParsedTime::Absolute(
                Utc::now()
                    .with_time(
                        NaiveTime::from_hms_opt(
                            raw_time.hour.into(),
                            raw_time.minute.into(),
                            raw_time.second.into(),
                        )
                        .unwrap(),
                    )
                    .unwrap()
                    .with_timezone(&tz),
            )
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use chrono_tz::Tz;

    use super::*;

    #[test]
    fn test_parse_time_with_utc_zone() {
        let (_, parsed_time) = parse_absolute_time("12:34:56 UTC").unwrap();

        let ParsedTime::Absolute(time) = parsed_time else {
            panic!("Unexpectedly returned a relative time");
        };

        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 34);
        assert_eq!(time.second(), 56);
        assert_eq!(time.timezone(), Tz::UTC);
    }

    #[test]
    fn test_parse_time_with_berlin_zone() {
        let (_, parsed_time) = parse_absolute_time("12:34:56 Europe/Berlin").unwrap();

        let ParsedTime::Absolute(time) = parsed_time else {
            panic!("Unexpectedly returned a relative time");
        };

        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 34);
        assert_eq!(time.second(), 56);
        assert_eq!(time.timezone(), Tz::Europe__Berlin);
    }
}
