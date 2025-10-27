use nom::{IResult, Parser, branch::alt, combinator::map};
use time::ParsedTime;

mod absolute;
mod relative;
mod time;

pub fn parse_timestring(input: &str) -> IResult<&str, ParsedTime> {
    alt((
        map(relative::parse_relative_time, |t| ParsedTime::Relative(t)),
        map(absolute::parse_absolute_time, |t| ParsedTime::Absolute(t)),
    ))
    .parse(input)
}
