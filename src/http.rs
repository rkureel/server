use std::{io::{BufRead, BufReader}, net::{SocketAddr, TcpListener, TcpStream}};

use crate::server::Server;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8080";

pub struct HttpServer {

}

impl HttpServer {
    pub fn new() -> Self {
        return HttpServer {};
    }
}

impl Server for HttpServer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let address: String = format!("{ADDRESS}:{PORT}");
        println!("Starting server on: [{address}]");

        let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;
        
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
        println!("Shutting down server");
        Ok(())
    }
}

fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    println!("Handling connection for address: {addr}");
    let mut buf: String = String::new();
    let mut reader: BufReader<TcpStream> = BufReader::new(stream);
    let n_bytes = reader.read_line(&mut buf).unwrap();
    if n_bytes == 0 as usize {
        println!("0 bytes read from from stream!");
        return;
    }


    dbg!(&buf.trim());
}
