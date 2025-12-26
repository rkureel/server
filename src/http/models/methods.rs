use std::fmt::Display;

use crate::http::models::errors::HttpRequestParsingError;

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    POST,
    PATCH,
    CONNECT,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Result<Self, HttpRequestParsingError> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "TRACE" => Ok(HttpMethod::TRACE),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "POST" => Ok(HttpMethod::POST),
            "PATCH" => Ok(HttpMethod::PATCH),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            other => Err(HttpRequestParsingError::InvalidHttpMethod(other.to_owned())),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::POST => "POST",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::CONNECT => "CONNECT",
        };
        write!(f, "{}", s)
    }
}
