use std::str::FromStr;

use nom::{
    Parser,
    character::complete::{alpha1, digit1, space0},
    combinator::map_res,
};
use strum_macros::EnumString;

use crate::error::FlexitimeResult2;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum RelativeUnitsError {
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),
}

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum RelativeUnit {
    #[strum(
        serialize = "seconds",
        serialize = "second",
        serialize = "secs",
        serialize = "sec",
        serialize = "s"
    )]
    Seconds,

    #[strum(
        serialize = "minutes",
        serialize = "minute",
        serialize = "mins",
        serialize = "min",
        serialize = "m"
    )]
    Minutes,

    #[strum(
        serialize = "hours",
        serialize = "hour",
        serialize = "hrs",
        serialize = "hr",
        serialize = "h"
    )]
    Hours,

    #[strum(serialize = "days", serialize = "day", serialize = "d")]
    Days,

    #[strum(
        serialize = "weeks",
        serialize = "week",
        serialize = "wks",
        serialize = "wk",
        serialize = "w"
    )]
    Weeks,

    #[strum(
        serialize = "months",
        serialize = "month",
        serialize = "mos",
        serialize = "mo"
    )]
    Months,

    #[strum(
        serialize = "years",
        serialize = "year",
        serialize = "yr",
        serialize = "y"
    )]
    Years,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUnit {
    pub unit: RelativeUnit,
    pub amount: u32,
}

fn parse_u32(input: &str) -> FlexitimeResult2<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>()).parse(input)
}

pub fn parse_unit(input: &str) -> FlexitimeResult2<&str, ParsedUnit> {
    let (input, amount) = parse_u32(input)?;

    let (input, _) = space0(input)?;

    let (input, token) = alpha1(input)?;
    let unit = RelativeUnit::from_str(token).map_err(|_err| {
        nom::Err::Error(RelativeUnitsError::UnknownUnit(token.to_string()).into())
    })?;

    Ok((input, ParsedUnit { amount, unit }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit() {
        assert_eq!(
            parse_unit("1h"),
            Ok((
                "",
                ParsedUnit {
                    amount: 1,
                    unit: RelativeUnit::Hours
                }
            ))
        );
        assert_eq!(
            parse_unit("2d"),
            Ok((
                "",
                ParsedUnit {
                    amount: 2,
                    unit: RelativeUnit::Days
                }
            ))
        );
        assert_eq!(
            parse_unit("3w"),
            Ok((
                "",
                ParsedUnit {
                    amount: 3,
                    unit: RelativeUnit::Weeks
                }
            ))
        );
        assert_eq!(
            parse_unit("4mo"),
            Ok((
                "",
                ParsedUnit {
                    amount: 4,
                    unit: RelativeUnit::Months
                }
            ))
        );
        assert_eq!(
            parse_unit("5y"),
            Ok((
                "",
                ParsedUnit {
                    amount: 5,
                    unit: RelativeUnit::Years
                }
            ))
        );
    }
}
