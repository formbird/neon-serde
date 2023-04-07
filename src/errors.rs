//! Defines error handling types used by the create
//! uses the `error-chain` create for generation

use std::convert::From;
use std::fmt::Display;

use neon;
use serde::{de, ser};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    /// nodejs has a hard coded limit on string length
    /// trying to serialize a string that is too long will result in an error
    #[error("String too long for nodejs len: {0}")]
    StringTooLong(usize),
    #[error("Unable to coerce value to type: {0}")]
    UnableToCoerce(&'static str),
    /// occurs when deserializing a char from an empty string
    #[error("EmptyString")]
    EmptyString,
    /// occurs when deserializing a char from a sting with
    /// more than one character
    #[error("String too long to be a char expected len: 1 got len: {0}")]
    StringTooLongForChar(usize),
    /// occurs when a deserializer expects a `null` or `undefined`
    /// property and found another type
    #[error("ExpectingNull")]
    ExpectingNull,
    /// occurs when deserializing to an enum and the source object has
    /// a none-1 number of properties
    #[error("key: '{0}'")]
    InvalidKeyType(String),
    /// an internal deserialization error from an invalid array
    #[error("ArrayIndexOutOfBounds: attempt to access ({0}) size: ({1})")]
    ArrayIndexOutOfBounds(u32, u32),
    /// This type of object is not supported
    #[error("Not Implemented {0}")]
    NotImplemented(&'static str),
    /// This type of object is not supported
    #[error("{0}")]
    DateError(#[from] neon::types::DateError),
    /// A JS exception was thrown
    #[error("JS exception")]
    Js(neon::result::Throw),
    /// failed to convert something to f64
    #[error("CastError")]
    CastError,
    #[error("{0}")]
    Msg(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Msg(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Msg(msg.to_string())
    }
}

impl From<neon::result::Throw> for Error {
    fn from(throw: neon::result::Throw) -> Self {
        Error::Js(throw)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[doc(hidden)]
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into())
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into())
    };
}
