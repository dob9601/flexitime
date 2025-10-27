use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlexitimeError {
    #[error("Invalid months")]
    InvalidMonths,

    #[error("Invalid years")]
    InvalidYears,
}

pub type FlexitimeResult<T> = Result<T, FlexitimeError>;
