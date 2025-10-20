use nom::{IResult, Parser, branch::alt};
use time::ParsedTime;

mod relative;
mod time;

pub fn parse_timestring(input: &str) -> IResult<&str, ParsedTime> {
    alt((relative::parse_relative_time,)).parse(input)
}
