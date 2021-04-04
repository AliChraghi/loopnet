use crate::thread::*;
use crate::utils::out;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

const DEFAULT_BUFFER_SIZE: usize = 2048;

pub struct HttpServer {
    ip: IpAddr,
    port: u16,
    workers: usize,
}

impl Default for HttpServer {
    fn default() -> Self {
        Self {
            ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8080,
            workers: 4,
        }
    }
}

impl HttpServer {
    pub fn new(ip: IpAddr, port: u16, workers: usize) -> Self {
        Self { ip, port, workers }
    }
    pub fn listen(&self) {
        let pool = ThreadPool::new(self.workers);
        match TcpListener::bind(SocketAddr::new(self.ip, self.port)) {
            Ok(listener) => {
                out::log(format!("Listening at {}:{}", self.ip, self.port).as_str());
                for stream in listener.incoming() {
                    let stream = stream.unwrap();
                    pool.execute(|| {
                        handle_connection(stream);
                    });
                }
            }
            Err(error) => out::error(error),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; DEFAULT_BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let status_line = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK\r\n\r\n"
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n"
    };

    stream.write(status_line.as_bytes()).unwrap();
    stream.flush().unwrap();
}
