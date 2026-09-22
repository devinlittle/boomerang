use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::LazyLock,
};

mod stuffies;
use anyhow::Error;
use stuffies::http::*;

pub struct Config {
    pub stun_server_address: Option<String>,
    pub stun_server_creds: Option<String>,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenvy::dotenv().ok();
    Config {
        stun_server_address: dotenvy::var("STUN_ADDR").ok(),
        // Credentials are USERNAME:PASSWORD
        stun_server_creds: dotenvy::var("STUN_CREDS").ok(),
    }
});

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("0.0.0.0:5252").unwrap();

    println!("Listening on 0.0.0.0:5252");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let remote_ip = stream.peer_addr()?;
                if let Err(err) = handle_req(stream, remote_ip) {
                    eprintln!("Error accepting connection: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
    Ok(())
}

fn handle_req(mut stream: TcpStream, remote_ip: SocketAddr) -> std::io::Result<()> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?; // Number of bytes in tcp stream

    let response = match parse_req(&buf[..n], remote_ip) {
        Ok(request) => send_response(request), // send_ip_back()
        Err(_) => Response::new_empty(),
    };

    stream.write_all(&response.to_bytes())?;
    stream.flush()?;
    Ok(())
}
