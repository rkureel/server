use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
};

use crate::http::models::{HttpMethod, HttpRequest, errors::HttpRequestParsingError};

pub fn read_request<T: Read>(stream: &mut T) -> Result<HttpRequest, Box<dyn Error>> {
    let mut reader: BufReader<&mut T> = BufReader::new(stream);
    let parsed_request_line = parse_request_line(&mut reader)?;

    let request: HttpRequest = HttpRequest {
        method: parsed_request_line.0,
        path: parsed_request_line.1,
        protocol: parsed_request_line.2,
    };
    Ok(request)
}

fn parse_request_line<T: BufRead>(
    reader: &mut T,
) -> Result<(HttpMethod, String, String), HttpRequestParsingError> {
    let mut request_line: String = String::new();
    reader.read_line(&mut request_line)?;

    let mut parts = request_line.split_whitespace();

    let (method, path, protocol) = match (parts.next(), parts.next(), parts.next()) {
        (Some(m), Some(pa), Some(pr)) => (m, pa, pr),
        _ => return Err(HttpRequestParsingError::InvalidRequestLine(request_line)),
    };

    return Ok((
        HttpMethod::from_str(method)?,
        path.to_owned(),
        protocol.to_owned(),
    ));
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::http::{models::HttpMethod, read::parse_request_line};

    #[test]
    fn successfully_parse_valid_request_line() {
        let request_line = "GET / HTTP/1.1";
        let mut reader = BufReader::new(request_line.as_bytes());
        let result = parse_request_line(&mut reader);

        assert!(result.is_ok());
        let (method, path, protocol) = result.unwrap();
        assert_eq!(method, HttpMethod::GET);
        assert_eq!(path, "/");
        assert_eq!(protocol, "HTTP/1.1");
    }

    #[test]
    fn return_error_for_invalid_request_line() {
        let request_line = "GET";
        let mut reader = BufReader::new(request_line.as_bytes());
        let result = parse_request_line(&mut reader);

        assert!(result.is_err());
    }
}
