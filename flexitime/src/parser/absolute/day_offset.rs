use chrono::Weekday;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::space1,
    combinator::{opt, value},
    sequence::preceded,
};

#[derive(Debug, Clone, PartialEq)]
pub enum DayOffset {
    Fixed(i32),
    NextDayOccurrence(Weekday),
}

pub fn parse_day_offset(input: &str) -> IResult<&str, DayOffset> {
    alt((
        value(DayOffset::Fixed(1), tag_no_case("tomorrow")),
        value(DayOffset::Fixed(-1), tag_no_case("yesterday")),
        preceded(
            opt((alt((tag_no_case("this"), tag_no_case("next"))), space1)),
            alt((
                value(
                    DayOffset::NextDayOccurrence(Weekday::Mon),
                    alt((tag_no_case("monday"), tag_no_case("mon"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Tue),
                    alt((tag_no_case("tuesday"), tag_no_case("tue"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Wed),
                    alt((tag_no_case("wednesday"), tag_no_case("wed"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Thu),
                    alt((tag_no_case("thursday"), tag_no_case("thu"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Fri),
                    alt((tag_no_case("friday"), tag_no_case("fri"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Sat),
                    alt((tag_no_case("saturday"), tag_no_case("sat"))),
                ),
                value(
                    DayOffset::NextDayOccurrence(Weekday::Sun),
                    alt((tag_no_case("sunday"), tag_no_case("sun"))),
                ),
            )),
        ),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_weekday() {
        assert_eq!(
            parse_day_offset("monday"),
            Ok(("", DayOffset::NextDayOccurrence(Weekday::Mon)))
        );
        assert_eq!(
            parse_day_offset("tue"),
            Ok(("", DayOffset::NextDayOccurrence(Weekday::Tue)))
        );
    }

    #[test]
    fn test_parse_weekday_with_prefix() {
        assert_eq!(
            parse_day_offset("this wednesday"),
            Ok(("", DayOffset::NextDayOccurrence(Weekday::Wed)))
        );
        assert_eq!(
            parse_day_offset("next thursday"),
            Ok(("", DayOffset::NextDayOccurrence(Weekday::Thu)))
        );
    }

    #[test]
    fn test_parse_yesterday() {
        assert_eq!(
            parse_day_offset("yesterday"),
            Ok(("", DayOffset::Fixed(-1)))
        );
    }

    #[test]
    fn test_parse_tomorrow() {
        assert_eq!(parse_day_offset("tomorrow"), Ok(("", DayOffset::Fixed(1))));
    }
}
