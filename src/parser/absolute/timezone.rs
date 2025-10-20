use chrono_tz::Tz;
use nom::{
    IResult, Parser,
    character::complete::{alpha1, space0},
    combinator::map_res,
    sequence::preceded,
};

pub fn parse_timezone(input: &str) -> IResult<&str, Tz> {
    preceded(space0, map_res(alpha1, |s: &str| s.parse::<Tz>())).parse(input)
}
