use nom::{IResult, Parser, branch::alt};
use time::ParsedTime;

mod absolute;
mod relative;
mod time;

pub fn parse_timestring(input: &str) -> IResult<&str, ParsedTime> {
    alt((relative::parse_relative_time, absolute::parse_absolute_time)).parse(input)
}
