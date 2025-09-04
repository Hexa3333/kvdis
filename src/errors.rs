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
    InvalidOperationType
}

#[derive(Debug)]
pub enum SerializationError {
    Key,
    Value
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
            }
        }
    }
}
