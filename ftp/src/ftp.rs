use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs;
use std::io;



pub fn connect_to_ftp_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))?;
    // If not control stream, we skip hello message
    if port != "21" {
        return Ok(stream);
    }
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
    let bytes_read = reader.read_line(&mut message)?;

    Ok(message)
}

fn write_message<T: Write>(write_stream: &mut T, message_to_sent: String) -> std::io::Result<()> {
    write_stream.write_all(message_to_sent.as_bytes())?;
    write_stream.flush()?;

    Ok(())
}

fn write_file_to_stream<T: Write>(write_stream: &mut T, file_path: &str) -> std::io::Result<()> {
    // Open the file for reading
    let mut file = File::open(file_path)?;

    let mut buffer = [0; 4096];

    // Read the data in the file and write it into the stream
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        write_stream.write_all(&buffer[0..bytes_read])?;
    }

    write_stream.flush()?;

    Ok(())
}

fn create_file_from_stream<T: Read>(read_stream: &mut T, file_path: &str) -> io::Result<()> {
    // Open/Create the file 
    let mut file = File::create(file_path)?;

    let mut buffer = [0; 4096];

    loop {
        let bytes_read = read_stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[0..bytes_read])?;

    }
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

// Login username and password
pub fn login(stream: &mut TcpStream, username: &str, password: &str) -> () {
    if let Ok(message) =  send_username_command(stream, username) {
        println!("{}", message);
    }
    if let Ok(message) = send_user_password_command(stream, password) {
        println!("{}", message);
    }
}

// Send username 
fn send_username_command(stream: &mut TcpStream, username: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("USER {}\r\n", username)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send password 
fn send_user_password_command(stream: &mut TcpStream, password: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("PASS {}\r\n", password)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

pub fn init_download_upload(stream: &mut TcpStream) -> () {
    if let Ok(message) = send_type_command(stream) {
        println!("{}", message);
    }
    if let Ok(message) = send_mode_command(stream) {
        println!("{}", message);
    }
    if let Ok(message) = send_stru_command(stream) {
        println!("{}", message);
    }
}

// Send TYPE command
fn send_type_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("TYPE I\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send MODE command
fn send_mode_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MODE S\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send STRU command
fn send_stru_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
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

// Send MKDIR command 
pub fn send_mkdir_command(stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MKD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send RMDIR command 
pub fn send_rmdir_command(stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("RMD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send REMOVE command 
pub fn send_remove_command(stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("DELE {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}

// Send PASV to create data channel
pub fn send_pasv_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("PASV\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err)
    };
    Ok(message)
}


// Send LIST command 
pub fn send_list_command(stream: &mut TcpStream, data_stream: &mut TcpStream, path:&str) -> Result<String, std::io::Error> {
    write_message(stream, format!("LIST {}\r\n", path))?;

    let message = match read_message(stream) {
        Ok(message) => {
            // Read and print the data from the data stream
            let mut data = String::new();
            data_stream.read_to_string(&mut data)?;
            // Print the data received from the data stream
            println!("Data received from data stream:\n{}", data);
            message
        }
        Err(err) => {
            return Err(err);
        }
    };

    Ok(message)
}

// Send COPY command
pub fn send_copy_command(stream: &mut TcpStream, data_stream: &mut TcpStream, source: &str, destination: &str, args: &[String]) -> Result<String, std::io::Error> {
    let mut message = String::new();
    if let Some(arg3) = args.get(3) {
        // logic for local cp to server
        if arg3.starts_with("ftp://") {
            // control stream write message
            write_message(stream, format!("STOR {}\r\n", source));

            write_file_to_stream(data_stream, source);

            message = match read_message(stream) {
                Ok(message) => message,
                Err(err) => {
                    return Err(err);
                }
            };
            return Ok(message);
        }
        // logic for server cp to local
        else {
            // control stream write message
            write_message(stream, format!("RETR {}\r\n", source));
            
            create_file_from_stream(data_stream, destination);
            message = match read_message(stream) {
                Ok(message) => message,
                Err(err) => {
                    return Err(err);
                }
            };
            return Ok(message);
        }
    }

    Ok(message)
}

// Send COPY command
pub fn send_move_command(stream: &mut TcpStream, data_stream: &mut TcpStream, source: &str, destination: &str, args: &[String]) -> Result<String, std::io::Error> {
    let mut message = String::new();
    if let Some(arg3) = args.get(3) {
        // logic for local mv to server
        if arg3.starts_with("ftp://") {
            // control stream write message
            write_message(stream, format!("STOR {}\r\n", source));

            write_file_to_stream(data_stream, source);

            // delete file in local
            match fs::remove_file(source) {
                Ok(_) => println!("File deleted successfully."),
                Err(err) => eprintln!("Error deleting file: {:?}", err),
            }

            message = match read_message(stream) {
                Ok(message) => message,
                Err(err) => {
                    return Err(err);
                }
            };
            return Ok(message);
        }
        // logic for server mv to local
        else {
            // control stream write message
            write_message(stream, format!("RETR {}\r\n", source));
            
            create_file_from_stream(data_stream, destination);

            // delete file on the ftp server
            write_message(stream, format!("DELE {}\r\n", source));

            message = match read_message(stream) {
                Ok(message) => message,
                Err(err) => {
                    return Err(err);
                }
            };
            return Ok(message);
        }
    }

    Ok(message)
}