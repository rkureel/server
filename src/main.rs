use std::error::Error;

use server::{http::HttpServer, server::Server};

fn main() -> Result<(), Box<dyn Error>> {
    

    let server: HttpServer = HttpServer::new();  
    server.start()?;

    Ok(())
}

