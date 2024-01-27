use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

// Define an enumeration to represent FTP commands
#[derive(Debug)]
pub enum Command {
    List(String),
    MakeDir(String),
    Remove(String),
    RemoveDir(String),
    Copy(String, String),
    Move(String, String),
}

// Function to parse command line arguments into an FTP command
pub fn parse_arguments(args: &[String]) -> Command {
    let command = &args[1];
    
    // Check if there are exactly 4 arguments (including the program name)
    if args.len() == 4 {
        let source_url = args[2].clone();
        let destination_url = args[3].clone();
        match command.as_str() {
            "cp" => return Command::Copy(source_url, destination_url),
            "mv" => return Command::Move(source_url, destination_url),

            _ => {
                println!("Invalud command: {command}");
                std::process::exit(1);
            }
        }
    }

    let server_url = args[2].clone();
    match command.as_str() {
        "ls" => Command::List(server_url),
        "mkdir" => Command::MakeDir(server_url),
        "rm" => Command::Remove(server_url),
        "rmdir" => Command::RemoveDir(server_url),

        _ => {
            println!("Invalud command: {command}");
            std::process::exit(1);
        }
    }
}

pub fn read_message<T: Read>(read_stream: &mut T) -> String {
    // Wrap the stream in a BufReader
    let mut reader = BufReader::new(read_stream);

    // Get the message
    let mut message = String::new();
    reader.read_line(&mut message);

    message
}

pub fn write_message<T: Write>(write_stream: &mut T, message_to_sent: String) -> std::io::Result<()> {
    write_stream.write_all(message_to_sent.as_bytes())?;
    write_stream.flush()?;

    Ok(())
}