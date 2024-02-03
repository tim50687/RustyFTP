use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs;
use std::io;
use std::env;

// Function to connect to an FTP server
pub fn connect_to_ftp_server(host: &str, port: &str) -> std::io::Result<TcpStream> {
    let mut stream = TcpStream::connect(format!("{host}:{port}"))?;

    // If not a data stream (port 21), skip the hello message
    if port != "21" {
        return Ok(stream);
    }

    // Read and print the server's hello message
    match read_message(&mut stream) {
        Ok(message) => println!("{message}"),
        Err(err) => eprintln!("Error reading message: {:?}", err),
    }

    Ok(stream)
}

// Function to read a message from a stream
fn read_message<T: Read>(read_stream: &mut T) -> Result<String, io::Error> {
    // Wrap the stream in a BufReader
    let mut reader = BufReader::new(read_stream);

    // Get the message
    let mut message = String::new();
    let bytes_read = reader.read_line(&mut message)?;

    Ok(message)
}

// Function to write a message to a stream
fn write_message<T: Write>(write_stream: &mut T, message_to_send: String) -> std::io::Result<()> {
    write_stream.write_all(message_to_send.as_bytes())?;
    write_stream.flush()?;

    Ok(())
}

// Function to write the contents of a file to a stream
fn write_file_to_stream<T: Write>(write_stream: &mut T, file_path: &str) -> std::io::Result<()> {
    // Open the file for reading, adding a "./" prefix if the path is not absolute
    let mut file_path_with_prefix = String::new();
    if !file_path.starts_with("/") {
        file_path_with_prefix = format!("./{}", file_path);
    } else {
        file_path_with_prefix = format!("{}", file_path);
    }
    let mut file = File::open(file_path_with_prefix)?;

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

// Function to create a file from data received from a stream
fn create_file_from_stream<T: Read>(read_stream: &mut T, file_path: &str) -> io::Result<()> {
    // Open or create the file
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

// Generic function to send FTP commands to the server
fn send_ftp_command(stream: &mut TcpStream, command: String) -> Result<String, std::io::Error> {
    write_message(stream, command)?;

    match read_message(stream) {
        Ok(message) => Ok(message),
        Err(err) => Err(err),
    }
}

// Function to log in with a username
fn send_username_command(stream: &mut TcpStream, username: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("USER {}\r\n", username)) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send a password
fn send_user_password_command(
    stream: &mut TcpStream,
    password: &str,
) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("PASS {}\r\n", password)) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to initialize FTP settings for download and upload
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

// Function to send TYPE command
fn send_type_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("TYPE I\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send MODE command
fn send_mode_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MODE S\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send STRU command
fn send_stru_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("STRU F\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send QUIT command
pub fn send_quit_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("QUIT\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send MKDIR command
pub fn send_mkdir_command(stream: &mut TcpStream, path: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("MKD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send RMDIR command
pub fn send_rmdir_command(stream: &mut TcpStream, path: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("RMD {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send REMOVE command
pub fn send_remove_command(stream: &mut TcpStream, path: &str) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("DELE {}\r\n", path)) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send PASV command to create a data channel
pub fn send_pasv_command(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let message = match send_ftp_command(stream, format!("PASV\r\n")) {
        Ok(message) => message,
        Err(err) => return Err(err),
    };
    Ok(message)
}

// Function to send LIST command
pub fn send_list_command(
    stream: &mut TcpStream,
    data_stream: &mut TcpStream,
    path: &str,
) -> Result<String, std::io::Error> {
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

// Function to send COPY command
pub fn send_copy_command(
    stream: &mut TcpStream,
    data_stream: &mut TcpStream,
    source: &str,
    destination: &str,
    args: &[String],
) -> Result<String, std::io::Error> {
    let mut message = String::new();
    if let Some(arg3) = args.get(3) {
        // Logic for local cp to server
        if arg3.starts_with("ftp://") {
            // Control stream writes message
            write_message(stream, format!("STOR {}\r\n", destination));

            match write_file_to_stream(data_stream, source) {
                Ok(_) => {
                    println!("File written successfully.");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }

            message = match read_message(stream) {
                Ok(message) => message,
                Err(err) => {
                    return Err(err);
                }
            };
            return Ok(message);
        }
        // Logic for server cp to local
        else {
            // Control stream writes message
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

// Function to send MOVE command
pub fn send_move_command(
    stream: &mut TcpStream,
    data_stream: &mut TcpStream,
    source: &str,
    destination: &str,
    args: &[String],
) -> Result<String, std::io::Error> {
    let mut message = String::new();
    if let Some(arg3) = args.get(3) {
        // Logic for local mv to server
        if arg3.starts_with("ftp://") {
            // Control stream writes message
            write_message(stream, format!("STOR {}\r\n", destination));

            match write_file_to_stream(data_stream, source) {
                Ok(_) => {
                    println!("File written successfully.");
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }

            // Delete file in local
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
        // Logic for server mv to local
        else {
            // Control stream writes message
            write_message(stream, format!("RETR {}\r\n", source));

            create_file_from_stream(data_stream, destination);

            // Delete file on the FTP server
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
