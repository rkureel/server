use std::{io::{BufRead, BufReader}, net::{SocketAddr, TcpListener, TcpStream}};

use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, Config};

use crate::server::Server;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8080";

pub struct HttpServer {

}

impl HttpServer {
    pub fn new() -> Self {
        return HttpServer {};
    }

    fn init_logger(&self) {
        let stdout: ConsoleAppender = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))
            .unwrap();

        let _handle = log4rs::init_config(config).unwrap();
    }
}

impl Server for HttpServer {
    fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.init_logger();

        let address: String = format!("{ADDRESS}:{PORT}");
        log::info!("Starting server on: [{address}]");

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


    dbg!(&buf.trim());
}
