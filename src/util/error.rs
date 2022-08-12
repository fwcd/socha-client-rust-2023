use std::io::Error as IoError;
use std::str::{ParseBoolError, Utf8Error};
use std::num::{ParseIntError, ParseFloatError};
use quick_xml::Error as XmlError;

use super::Element;

/// A custom error type that abstracts over
/// other errors (such as IO/XML errors) and
/// can conveniently be used in conjunction with
/// `Result`.
#[derive(Debug)]
pub enum SCError {
    Io(IoError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    ParseBool(ParseBoolError),
    Utf8(Utf8Error),
    Xml(XmlError),
    UnknownElement(Element),
    UnknownVariant(String),
    InvalidState(String),
    ServerError(String),
    Eof,
    Custom(String)
}

impl From<IoError> for SCError {
    fn from(error: IoError) -> Self { Self::Io(error) }
}

impl From<ParseIntError> for SCError {
    fn from(error: ParseIntError) -> Self { Self::ParseInt(error) }
}

impl From<ParseFloatError> for SCError {
    fn from(error: ParseFloatError) -> Self { Self::ParseFloat(error) }
}

impl From<ParseBoolError> for SCError {
    fn from(error: ParseBoolError) -> Self { Self::ParseBool(error) }
}

impl From<Utf8Error> for SCError {
    fn from(error: Utf8Error) -> Self { Self::Utf8(error) }
}

impl From<XmlError> for SCError {
    fn from(error: XmlError) -> Self { Self::Xml(error) }
}

impl From<String> for SCError {
    fn from(error: String) -> Self { Self::Custom(error) }
}

impl<'a> From<&'a str> for SCError {
    fn from(error: &'a str) -> Self { Self::Custom(error.to_owned()) }
}
