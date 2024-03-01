use std::{
    convert::Infallible,
    num::{ParseFloatError, ParseIntError},
};

pub type Result<T> = std::result::Result<T, TailwindError>;

#[derive(Debug)]
pub enum TailwindError {
    Unexpected(String),
    Unimplemented(String),
    Syntax(String),
    NomError(String),
}

impl TailwindError {
    pub fn syntax_error(message: impl Into<String>) -> Self {
        TailwindError::Syntax(message.into())
    }
}

impl<I> From<nom::Err<(I, nom::error::ErrorKind)>> for TailwindError
where
    I: std::fmt::Debug,
{
    fn from(err: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        TailwindError::NomError(format!("{:?}", err))
    }
}

use nom::Err as NomErr;
impl From<NomErr<nom::error::Error<&str>>> for TailwindError {
    fn from(err: NomErr<nom::error::Error<&str>>) -> Self {
        TailwindError::NomError(format!("Nom parsing error: {:?}", err))
    }
}

impl From<Infallible> for TailwindError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<()> for TailwindError {
    fn from(_: ()) -> Self {
        unreachable!()
    }
}

impl From<ParseIntError> for TailwindError {
    fn from(e: ParseIntError) -> Self {
        Self::syntax_error(e.to_string())
    }
}

impl From<ParseFloatError> for TailwindError {
    fn from(e: ParseFloatError) -> Self {
        Self::syntax_error(e.to_string())
    }
}
