use std::fmt::Display;

#[derive(Debug)]
pub struct CustomError {
    msg: String,
}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for CustomError {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    OnebootError(CustomError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    PestParseRestError(#[from] pest::error::Error<crate::restful::Rule>),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    InfallibleError(#[from] std::convert::Infallible),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Into<Error> for &str {
    fn into(self) -> Error {
        Error::OnebootError(CustomError {
            msg: self.to_string(),
        })
    }
}
