use nom::error::ParseError;
use thiserror::Error;

use crate::parser::{
    absolute::{date::AbsoluteDateError, wallclock_time::WallClockTimeError},
    relative::units::RelativeUnitsError,
};

#[derive(Debug, Error)]
pub enum FlexitimeError {
    #[error("Invalid months")]
    InvalidMonths,

    #[error("Invalid years")]
    InvalidYears,
}

pub type FlexitimeResult<T> = Result<T, FlexitimeError>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum FlexitimeError2<I> {
    DayOffsetParse,
    ParseIntError(#[from] std::num::ParseIntError),
    WallClockTime(#[from] WallClockTimeError),
    RelativeUnits(#[from] RelativeUnitsError),
    Date(#[from] AbsoluteDateError),
    Nom(I, nom::error::ErrorKind),
}

impl<I> ParseError<I> for FlexitimeError2<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        FlexitimeError2::Nom(input, kind)
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> From<nom::error::Error<I>> for FlexitimeError2<I> {
    fn from(err: nom::error::Error<I>) -> Self {
        FlexitimeError2::Nom(err.input, err.code)
    }
}

impl<I, E> nom::error::FromExternalError<I, E> for FlexitimeError2<I>
where
    FlexitimeError2<I>: From<E>,
{
    fn from_external_error(_input: I, _kind: nom::error::ErrorKind, e: E) -> Self {
        Self::from(e)
    }
}

pub type FlexitimeResult2<I, O> = nom::IResult<I, O, FlexitimeError2<I>>;
