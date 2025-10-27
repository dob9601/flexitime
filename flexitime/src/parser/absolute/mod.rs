use chrono::NaiveDate;
pub use day_offset::DayOffset;
use nom::{
    IResult, Parser, branch::alt, character::complete::space0, combinator::map, multi::fold_many1,
    sequence::delimited,
};
pub use wallclock_time::{TimePeriod, WallClockTime};

mod day_offset;
mod time;
pub use time::{AbsoluteTime, AbsoluteTimeBuilder, FlexiDate};
mod date;
mod wallclock_time;

pub enum AbsoluteTimePart {
    DayOffset(DayOffset),
    Date(NaiveDate),
    WallClockTime(WallClockTime),
}

pub fn parse_absolute_time(input: &str) -> IResult<&str, AbsoluteTime> {
    fold_many1(
        delimited(
            space0,
            alt((
                map(day_offset::parse_day_offset, AbsoluteTimePart::DayOffset),
                map(
                    wallclock_time::parse_wall_clock_time,
                    AbsoluteTimePart::WallClockTime,
                ),
                map(date::parse_date, AbsoluteTimePart::Date),
            )),
            space0,
        ),
        || AbsoluteTimeBuilder::new(),
        |acc, part| match part {
            AbsoluteTimePart::DayOffset(offset) => acc.date(FlexiDate::DayOffset(offset)),
            AbsoluteTimePart::Date(date) => acc.date(FlexiDate::Date(date)),
            AbsoluteTimePart::WallClockTime(wall_clock_time) => acc.time(wall_clock_time),
        },
    )
    .map(|builder| builder.build().unwrap())
    .parse(input)
}

#[cfg(test)]
mod tests {
    use chrono::Weekday;

    use crate::parser::absolute::wallclock_time::TimePeriod;

    use super::*;

    #[test]
    fn test_parse_time_and_day_offset() {
        assert_eq!(
            parse_absolute_time("tuesday 9:00pm").unwrap(),
            (
                "",
                AbsoluteTimeBuilder::new()
                    .date(FlexiDate::DayOffset(DayOffset::NextDayOccurrence(
                        Weekday::Tue
                    )))
                    .time(WallClockTime::new(9, 0, 0, Some(TimePeriod::Pm)).unwrap())
                    .build()
                    .unwrap()
            )
        )
    }
    #[test]
    fn test_parse_date_and_time() {
        assert_eq!(
            parse_absolute_time("25/10/2025 11:25am").unwrap(),
            (
                "",
                AbsoluteTimeBuilder::new()
                    .date(FlexiDate::Date(
                        NaiveDate::from_ymd_opt(2025, 10, 25).unwrap()
                    ))
                    .time(WallClockTime::new(11, 25, 0, Some(TimePeriod::Am)).unwrap())
                    .build()
                    .unwrap()
            )
        )
    }
}
