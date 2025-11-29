use crate::{
    http::{models::HttpRequest, read::read_request},
    server::{Server, config::ServerConfig},
};
use std::{
    io::BufReader,
    net::{SocketAddr, TcpListener, TcpStream},
};

pub struct HttpServer {
    server_config: ServerConfig,
}

impl HttpServer {
    pub fn new(server_config: ServerConfig) -> Self {
        return HttpServer {
            server_config: server_config,
        };
    }
}

impl Server for HttpServer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let address: String = self.server_config.get_address();
        log::info!("Starting server on: [{address}]");

        let listener: TcpListener = TcpListener::bind(&address)?;

        loop {
            match listener.accept() {
                Ok((_stream, addr)) => {
                    handle_connection(_stream, addr);
                }
                Err(e) => println!("Couldn't handle connection: {e}"),
            }
        }

        Ok(())
    }

    fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Shutting down server");
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, addr: SocketAddr) {
    log::info!("Handling connection for address: {addr}");
    let request: HttpRequest = read_request(&mut stream).unwrap();
    dbg!(&request);
}
