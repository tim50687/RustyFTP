use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::io;



pub fn connect_to_ftp_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))?;

    match read_message(&mut stream) {
        Ok(message) => println!("{message}"),
        Err(err) => eprintln!("Error reading message: {:?}", err),
    }

    Ok(stream)
}

fn read_message<T: Read>(read_stream: &mut T) -> Result<String, io::Error> {
    // Wrap the stream in a BufReader
    let mut reader = BufReader::new(read_stream);

    // Get the message
    let mut message = String::new();
    reader.read_line(&mut message)?;

    Ok(message)
}
fn write_message<T: Write>(write_stream: &mut T, message_to_sent: String) -> std::io::Result<()> {
    write_stream.write_all(message_to_sent.as_bytes())?;
    write_stream.flush()?;

    Ok(())
}