use chrono_tz::Tz;
use nom::{IResult, Parser, bytes::complete::take_while1, combinator::map_res};

pub fn parse_timezone(input: &str) -> IResult<&str, Tz> {
    map_res(take_while1(|c: char| !c.is_whitespace()), |s: &str| {
        s.parse::<Tz>()
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_utc() {
        assert_eq!(parse_timezone("UTC"), Ok(("", Tz::UTC)));
    }

    #[test]
    fn test_parse_berlin() {
        assert_eq!(
            parse_timezone("Europe/Berlin"),
            Ok(("", Tz::Europe__Berlin))
        );
    }
}
