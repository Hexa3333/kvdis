#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidParameters,
    NotACommand
}

#[derive(Debug, PartialEq, Eq)]
pub enum DictionaryError {
    DoesNotExist,
    IsExpired
}
