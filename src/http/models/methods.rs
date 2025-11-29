use std::fmt::Display;

#[derive(Debug)]
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
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => HttpMethod::GET,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "CONNECT" => HttpMethod::CONNECT,
            other => panic!("Invalid Http method: {other}"),
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
