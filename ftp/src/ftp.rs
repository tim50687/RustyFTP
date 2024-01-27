use std::net::TcpStream;

use crate::utils::read_message;



pub fn connect_to_ftp_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))?;

    let message = read_message(&mut stream);
    println!("{message}");

    Ok(stream)
}