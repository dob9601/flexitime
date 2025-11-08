use chrono::Month;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::space1,
    combinator::{opt, value},
    sequence::preceded,
};

use crate::error::FlexitimeResult;

#[derive(Debug, Clone, PartialEq)]
pub enum MonthOffset {
    NextMonthOccurrence(Month),
}

pub fn parse_month_offset(input: &str) -> FlexitimeResult<&str, MonthOffset> {
    preceded(
        opt((alt((tag_no_case("this"), tag_no_case("next"))), space1)),
        alt((
            value(
                MonthOffset::NextMonthOccurrence(Month::January),
                alt((tag_no_case("january"), tag_no_case("jan"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::February),
                alt((tag_no_case("february"), tag_no_case("feb"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::March),
                alt((tag_no_case("march"), tag_no_case("mar"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::April),
                alt((tag_no_case("april"), tag_no_case("apr"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::May),
                tag_no_case("may"),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::June),
                alt((tag_no_case("june"), tag_no_case("jun"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::July),
                alt((tag_no_case("july"), tag_no_case("jul"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::August),
                alt((tag_no_case("august"), tag_no_case("aug"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::September),
                alt((tag_no_case("september"), tag_no_case("sep"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::October),
                alt((tag_no_case("october"), tag_no_case("oct"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::November),
                alt((tag_no_case("november"), tag_no_case("nov"))),
            ),
            value(
                MonthOffset::NextMonthOccurrence(Month::December),
                alt((tag_no_case("december"), tag_no_case("dec"))),
            ),
        )),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_month_offset() {
        assert_eq!(
            parse_month_offset("december"),
            Ok(("", MonthOffset::NextMonthOccurrence(Month::December)))
        )
    }

    #[test]
    fn test_parse_month_offset_with_prefix() {
        assert_eq!(
            parse_month_offset("this january"),
            Ok(("", MonthOffset::NextMonthOccurrence(Month::January)))
        )
    }
}
