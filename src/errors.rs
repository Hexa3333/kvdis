use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidParameters,
    NotACommand,
    IsEmpty
}

#[derive(Debug, PartialEq, Eq)]
pub enum DictionaryError {
    DoesNotExist,
    IsExpired,
    InvalidOperationType,

    IOError(SerializationError)
}

#[derive(Debug, PartialEq, Eq)]
pub enum SerializationError {
    KeyRead,
    ValueRead,
    TimestampRead,

    IORead,
    IOWrite
}

// Uhm...
impl From<SerializationError> for DictionaryError {
    fn from(value: SerializationError) -> Self {
        match value {
            SerializationError::IORead => DictionaryError::IOError(SerializationError::IORead),
            SerializationError::IOWrite => DictionaryError::IOError(SerializationError::IOWrite),
            _ => unreachable!()
        }
    }
}

impl Display for ParseError  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NotACommand => write!(f, "Not a command!"),
            ParseError::InvalidParameters => write!(f, "Command parameters are invalid!"),
            ParseError::IsEmpty => write!(f, "Empty")
        }
    }
}

impl Display for DictionaryError  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::DoesNotExist => write!(f, "Key does not exist."),
            DictionaryError::IsExpired => write!(f, "Key has expired."),
            DictionaryError::InvalidOperationType => write!(f, "This operation is not defined on value type."),
            DictionaryError::IOError(e) => write!(f, "{e}")
        }
    }
}

impl Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializationError::KeyRead => write!(f, "Key could not be read."),
            SerializationError::ValueRead => write!(f, "Value could not be read."),
            SerializationError::TimestampRead => write!(f, "Expiration timestamp could not be read."),
            SerializationError::IORead => write!(f, "IO read failed."),
            SerializationError::IOWrite => write!(f, "IO write failed.")
        }
    }
}
