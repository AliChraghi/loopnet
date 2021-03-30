// use crate::router::Router;
use crate::utils::*;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct HttpServer {
    ip: [u8; 4],
    port: u16,
}

impl HttpServer {
    pub fn new(ip: [u8; 4], port: u16) -> HttpServer {
        return HttpServer { ip, port };
    }
    pub fn listen(&self) {
        match TcpListener::bind(SocketAddr::from((self.ip, self.port))) {
            Ok(listener) => {
                let addr = format!(
                    "{}:{}",
                    self.ip
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join("."),
                    self.port.to_string()
                );
                log(format!("Server Started At {}", &addr).as_str());
                for stream in listener.incoming() {
                    match stream {
                        Ok(s) => handle_connection(s),
                        Err(e) => log_error(format!("{}", e).as_str())
                    }
                }
            }
            Err(e) => log_error(format!("{}", e).as_str()),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let status_line = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else if buffer.starts_with(sleep) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n"
    };

    let response = status_line;
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
