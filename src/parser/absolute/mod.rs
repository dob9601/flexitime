use chrono::{DateTime, TimeZone, Utc};
use nom::{IResult, Parser};

use super::time::ParsedTime;

mod timezone;

pub fn parse_absolute_time(input: &str) -> IResult<&str, ParsedTime> {
    let datetime = Utc::now(); // TODO

    let (input, timezone) = timezone::parse_timezone(input)?;

    timezone.with_ymd_and_hms();

    todo!();
    // Ok((
    //     input,
    //     ParsedTime::Absolute(timezone datetime.with_timezone(timezone)),
    // ))
}
