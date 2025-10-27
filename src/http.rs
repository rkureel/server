use std::{io::{BufRead, BufReader}, net::{SocketAddr, TcpListener, TcpStream}};
use log::debug;

use crate::server::{config::ServerConfig, Server};


pub struct HttpServer {
    server_config: ServerConfig
}

impl HttpServer {
    pub fn new(server_config: ServerConfig) -> Self {
        return HttpServer {
            server_config: server_config
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
                Err(e) => println!("Couldn't handle connection: {e}")
            }
        }

        Ok(()) 
    }

    fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Shutting down server");
        Ok(())
    }
}

fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    log::info!("Handling connection for address: {addr}");
    let mut buf: String = String::new();
    let mut reader: BufReader<TcpStream> = BufReader::new(stream);
    let n_bytes = reader.read_line(&mut buf).unwrap();
    if n_bytes == 0 as usize {
        log::debug!("0 bytes read from from stream!");
        return;
    }
}
