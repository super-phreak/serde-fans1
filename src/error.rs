use std::io;
use std::result;

pub struct Error {
    // This `Box` allows us to keep the size of `Error` as small as possible. A
    // larger `Error` type was substantially slower due to all the functions
    // that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>
}

struct ErrorImpl {
    code: ErrorCode,
    word: usize,
}

// To assume the correct ASN1 Error type
pub type Result<T> = result::Result<T, Error>;

impl Error {

    pub fn word(&self) -> usize {
        self.err.word
    }
}

pub(crate) enum ErrorCode {
    Message(Box<str>),
    Io(Box<io::Error>),
    EofWhileParsingValue,
    InvalidTag,
    InvalidLength, 
    
} 