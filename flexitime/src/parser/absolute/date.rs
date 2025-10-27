use chrono::NaiveDate;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::char,
    combinator::map_res,
    error::{Error, ErrorKind},
};

fn date_delimiter(input: &str) -> IResult<&str, char> {
    alt((char('-'), char('/'))).parse(input)
}

fn parse_year(input: &str) -> IResult<&str, u16> {
    map_res(
        take_while_m_n(4, 4, |c: char| c.is_ascii_digit()),
        |s: &str| {
            s.parse::<u16>()
                .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))
                .and_then(|year| {
                    if year > 3000 {
                        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Digit)));
                    }

                    Ok(year)
                })
        },
    )
    .parse(input)
}

fn parse_day(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(1, 2, |c: char| c.is_ascii_digit()),
        |s: &str| {
            s.parse::<u8>()
                .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))
                .and_then(|day| {
                    if day > 31 {
                        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Digit)));
                    }

                    Ok(day)
                })
        },
    )
    .parse(input)
}

fn parse_month(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(1, 2, |c: char| c.is_ascii_digit()),
        |s: &str| {
            s.parse::<u8>()
                .map_err(|_| nom::Err::Error(Error::new(input, ErrorKind::Digit)))
                .and_then(|month| {
                    if month > 12 {
                        return Err(nom::Err::Failure(Error::new(input, ErrorKind::Digit)));
                    }

                    Ok(month)
                })
        },
    )
    .parse(input)
}

pub fn parse_date(input: &str) -> IResult<&str, NaiveDate> {
    alt((parse_day_month_year, parse_year_month_day)).parse(input)
}

fn parse_day_month_year(input: &str) -> IResult<&str, NaiveDate> {
    map_res(
        (
            parse_day,
            date_delimiter,
            parse_month,
            date_delimiter,
            parse_year,
        ),
        |(day, _, month, _, year)| {
            NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).ok_or(
                nom::Err::Failure::<Error<&str>>(Error::new(input, ErrorKind::Fail)),
            )
        },
    )
    .parse(input)
}

fn parse_year_month_day(input: &str) -> IResult<&str, NaiveDate> {
    map_res(
        (
            parse_year,
            date_delimiter,
            parse_month,
            date_delimiter,
            parse_day,
        ),
        |(year, _, month, _, day)| {
            NaiveDate::from_ymd_opt(year.into(), month.into(), day.into()).ok_or(
                nom::Err::Failure::<Error<&str>>(Error::new(input, ErrorKind::Fail)),
            )
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ymd() {
        assert_eq!(
            parse_date("2023-01-01"),
            Ok(("", NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()))
        );
        assert_eq!(
            parse_date("2029/01/01"),
            Ok(("", NaiveDate::from_ymd_opt(2029, 1, 1).unwrap()))
        );
    }

    #[test]
    fn test_parse_dmy() {
        assert_eq!(
            parse_date("01-01-2023"),
            Ok(("", NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()))
        );
        assert_eq!(
            parse_date("01/01/2029"),
            Ok(("", NaiveDate::from_ymd_opt(2029, 1, 1).unwrap()))
        );
    }
}
