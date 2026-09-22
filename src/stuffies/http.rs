use std::{collections::HashMap, net::SocketAddr};

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

    let user_agent = find_user_agent(raw_data);

    // INFO:: put the line of code below when stun is implimented
    //
    //let ip = if CONFIG.stun_server_address.is_none() || !is_private_ip(ip) {

    let ip = if !is_private_ip(ip) {
        ip.ip().to_string()
    } else {
        // TODO: impliment stun ip lookup here
        format!(
            "PRIVATE: {}\nPUBLIC: WILL IMPLIMENT FINDING PUBLIC IP LATER",
            ip.ip()
        )
    };

    Ok(Request {
        method,
        path,
        version,
        ip,
        user_agent,
    })
}

fn find_user_agent(raw_request_data: &str) -> String {
    let split = raw_request_data.split("\r\n");
    for i in split.enumerate() {
        if i.1.contains("User-Agent") {
            return i.1.to_string();
        }
    }
    "User-Agent not found".to_string()
}

fn is_private_ip(ip: SocketAddr) -> bool {
    if ip.is_ipv6() {
        return false;
    }

    let ipv4 = match ip.ip() {
        std::net::IpAddr::V4(ipv4) => ipv4,
        std::net::IpAddr::V6(_) => unreachable!(),
    }
    .octets();

    dbg!("{:?}", ipv4);

    // 10.xxx.xxx.xxx
    if ipv4[0] == 10 {
        return true;
    }

    // 192.168.xxx.xxx
    if ipv4[0] == 192 && ipv4[1] == 168 {
        return true;
    }

    // 172.16.000.000 -> 172.31.255.255
    if ipv4[0] == 172 && ipv4[1] >= 16 && ipv4[1] <= 31 {
        return true;
    }

    false
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
    let data = format!("{}\n\n{}", request.ip, request.user_agent).into_bytes();
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
