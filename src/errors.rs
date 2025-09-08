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

impl ToString for ParseError {
    fn to_string(&self) -> String {
        match self {
            ParseError::NotACommand => {
                "Not a command!".to_string()
            },
            ParseError::InvalidParameters => {
                "Command parameters are invalid!".to_string()
            },
            ParseError::IsEmpty => {
                "Empty.".to_string()
            }
        }
    }
}

impl ToString for DictionaryError {
    fn to_string(&self) -> String {
        match self {
            DictionaryError::DoesNotExist => {
                "Key does not exist.".to_string()
            },
            DictionaryError::IsExpired => {
                "Key has expired.".to_string()
            },
            DictionaryError::InvalidOperationType => {
                "This operation is not defined on value type.".to_string()
            },
            DictionaryError::IOError(e) => {
                e.to_string()
            }
        }
    }
}

impl ToString for SerializationError {
    fn to_string(&self) -> String {
        match self {
            SerializationError::KeyRead => {
                "Key could not be read.".to_string()
            },
            SerializationError::ValueRead => {
                "Value could not be read.".to_string()
            },
            SerializationError::TimestampRead => {
                "Expiration timestamp could not be read.".to_string()
            },
            SerializationError::IORead => {
                "IO read failed.".to_string()
            },
            SerializationError::IOWrite => {
                "IO write failed.".to_string()
            }
        }
    }
}
