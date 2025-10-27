use std::error::Error;

use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, Config};
use server::{http::HttpServer, server::{config::ServerConfig, Server}};

const ADDRESS: &str = "127.0.0.1";
const PORT: u32 = 8080;

fn main() -> Result<(), Box<dyn Error>> {
    init_logger();
     
    let server_config: ServerConfig = ServerConfig::new(ADDRESS, PORT);

    let server: HttpServer = HttpServer::new(server_config);  
    server.start()?;

    Ok(())
}

fn init_logger() {
    let stdout: ConsoleAppender = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}
