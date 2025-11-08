use nom::error::ParseError;
use thiserror::Error;

use crate::parser::{
    absolute::{date::AbsoluteDateError, wallclock_time::WallClockTimeError},
    relative::units::RelativeUnitsError,
};

#[derive(Debug, PartialEq, Error)]
pub enum FlexitimeError<I> {
    DayOffsetParse,
    ParseIntError(#[from] std::num::ParseIntError),
    WallClockTime(#[from] WallClockTimeError),
    RelativeUnits(#[from] RelativeUnitsError),
    Date(#[from] AbsoluteDateError),
    Nom(I, nom::error::ErrorKind),
}

impl<I> ParseError<I> for FlexitimeError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        FlexitimeError::Nom(input, kind)
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> From<nom::error::Error<I>> for FlexitimeError<I> {
    fn from(err: nom::error::Error<I>) -> Self {
        FlexitimeError::Nom(err.input, err.code)
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for FlexitimeError<I>
where
    FlexitimeError<I>: From<E>,
{
    fn from_external_error(_input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        Self::from(e)
    }
}

pub type FlexitimeResult<I, O> = nom::IResult<I, O, FlexitimeError<I>>;
