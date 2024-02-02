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

// Generic function to send commands to ftp server
fn send_ftp_command(stream: &mut TcpStream, command: String) -> Result<String, std::io::Error> {
    write_message(stream, command)?;

    match read_message(stream) {
        Ok(message) => Ok(message),
        Err(err) => {
            Err(err)
        }
    }
}

// Send username 
pub fn send_username_command(stream: &mut TcpStream, username: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("USER {}\r\n", username)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send password 
pub fn send_user_password_command(stream: &mut TcpStream, password: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("PASS {}\r\n", password)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send TYPE command
pub fn send_type_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("TYPE I\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send MODE command
pub fn send_mode_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MODE S\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send STRU command
pub fn send_stru_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("STRU F\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send QUIT command
pub fn send_quit_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("QUIT\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// send MKDIR command 
pub fn send_mkdir_command(stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MKD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// send RMDIR command 
pub fn send_rmdir_command(stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("RMD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}