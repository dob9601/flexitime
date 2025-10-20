use nom::{
    IResult, Parser,
    branch::alt,
    bytes::tag_no_case,
    character::complete::space1,
    combinator::{opt, value},
    sequence::preceded,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suffix {
    Ago,
    Hence,
}

pub fn parse_suffix(input: &str) -> IResult<&str, Option<Suffix>> {
    opt(preceded(
        space1,
        alt((
            value(Suffix::Ago, tag_no_case("ago")),
            value(Suffix::Hence, tag_no_case("hence")),
        )),
    ))
    .parse(input)
}
