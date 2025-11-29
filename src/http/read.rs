use std::{
    error::Error,
    io::{BufRead, BufReader, Read},
    str::SplitAsciiWhitespace,
};

use crate::http::models::{HttpMethod, HttpRequest};

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
) -> Result<(HttpMethod, String, String), Box<dyn Error>> {
    let mut request_line: String = String::new();
    reader.read_line(&mut request_line)?;

    let mut request_line_iter = request_line.split_whitespace();
    let method: HttpMethod = HttpMethod::from_str(&request_line_iter.next().unwrap());
    let request_path = String::from(request_line_iter.next().unwrap());
    let protocol = String::from(request_line_iter.next().unwrap());
    return Ok((method, request_path, protocol));
}
