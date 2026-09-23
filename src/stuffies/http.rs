use std::{collections::HashMap, net::SocketAddr};

use crate::CONFIG;

#[derive(Debug)]
pub enum ParseError {
    Empty,
    MalformedRequestLine,
    InvalidUtf8,
}

pub struct Request {
    #[allow(dead_code)]
    method: String,
    #[allow(dead_code)]
    path: String,
    version: String,
    ip: String,
    user_agent: String,
}

const REVERSE_PROXY_HEADERS: [&str; 3] = ["X-Forwarded-For", "X-Real-Ip", "X-Real-IP"];

pub fn parse_req(buf: &[u8], ip: SocketAddr) -> Result<Request, ParseError> {
    let raw_data = std::str::from_utf8(buf).map_err(|_| ParseError::InvalidUtf8)?;
    let req_line = raw_data.split("\r\n").next().ok_or(ParseError::Empty)?;
    let mut parts = req_line.splitn(3, " ");
    let method = parts
        .next()
        .ok_or(ParseError::MalformedRequestLine)?
        .to_string();
    let path = parts
        .next()
        .ok_or(ParseError::MalformedRequestLine)?
        .to_string();
    let version = parts
        .next()
        .ok_or(ParseError::MalformedRequestLine)?
        .to_string();

    let user_agent = find_first_header(raw_data, &["User-Agent"])
        .unwrap_or("User-Agent could not be found")
        .to_string();

    let ip = if CONFIG.reverse_proxy {
        match find_first_header(raw_data, &REVERSE_PROXY_HEADERS) {
            Some(ip) => ip
                .rfind(": ")
                .map(|x| &ip[x + 2..])
                .unwrap_or("ip not found?")
                .to_string(),
            None => "ip not found?".to_string(),
        }
    } else {
        ip.ip().to_string()
    };

    Ok(Request {
        method,
        path,
        version,
        ip,
        user_agent,
    })
}

fn find_first_header<'a>(raw_request_data: &'a str, headers: &[&str]) -> Option<&'a str> {
    raw_request_data.split("\r\n").find(|&header| {
        headers
            .iter()
            .any(|&h| header.starts_with(h) || header.eq_ignore_ascii_case(h))
    })
}

pub struct Response {
    version: String,
    status_code: u16,
    reason: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new_empty() -> Response {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Type".to_string(), "type/html".to_string());
        headers.insert("Content-Length".to_string(), "0".to_string());
        headers.insert("Server".to_string(), "ByteLittle".to_string());
        let empty_data: Vec<u8> = Vec::new();

        Response {
            version: "HTTP/1.1".to_string(),
            status_code: 418,
            reason: "IM A TEAPOT".to_string(),
            headers,
            body: empty_data,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut headers: String = String::new();
        for (name, val) in self.headers.iter() {
            headers = headers + &format!("{name}: {val}\r\n").to_string();
        }

        let mut response = format!(
            "{} {} {}\r\n{}Server: ByteLittle\r\n\r\n",
            self.version, self.status_code, self.reason, headers
        )
        .into_bytes();
        response.extend_from_slice(&self.body);
        response
    }
}

pub fn send_response(request: Request) -> Response {
    let data = format!("{}\n\n{}\n", request.ip, request.user_agent).into_bytes();
    let data_length = &data.len().to_string();

    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Length".to_string(), data_length.clone());

    Response {
        version: request.version,
        status_code: 200,
        reason: "OK".to_string(),
        headers,
        body: data,
    }
}
