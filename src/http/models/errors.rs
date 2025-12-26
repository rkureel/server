use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpRequestParsingError {
    #[error("Invalid HTTP request line: [0]")]
    InvalidRequestLine(String),

    #[error("Invalid HTTP method: [0]")]
    InvalidHttpMethod(String),

    #[error("IO error while reading request")]
    IOError(#[from] io::Error),
}
