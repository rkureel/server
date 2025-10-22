use std::{error::Error, net::{SocketAddr, TcpListener, TcpStream}};

fn main() -> Result<(), Box<dyn Error>> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Starting server");

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

fn handle_connection(_stream: TcpStream, addr: SocketAddr) {
    println!("Handling connection for address: {addr}");
}
