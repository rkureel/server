use std::{error::Error, io::{BufRead, BufReader}, net::{SocketAddr, TcpListener, TcpStream}};

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
