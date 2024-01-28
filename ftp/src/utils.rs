use url::Url;
use std::error::Error;

const PORT: &str = "21";

// Define an enumeration to represent FTP commands
// (username, password, url, <url>)
#[derive(Debug)]
pub enum Command {
    List(String, String, String),
    MakeDir(String, String, String),
    Remove(String, String, String),
    RemoveDir(String, String, String),
    Copy(String, String, String, String),
    Move(String, String, String, String),
}

// Function to parse command line arguments into an FTP command
pub fn parse_arguments(args: &[String]) -> Command {
    let command = &args[1];
    let username;
    let password;

    // Get username and password
    match get_username(&args[2]) {
        Ok(_username) => {
            username = _username;
        }
        _ => {
            username = "".to_string();
        }
    }
    match get_password(&args[2]) {
        Ok(Some(_password)) => {
            password = _password;
        }
        _ => {
            password = "".to_string();   
        }
    }
    
    // Check if there are exactly 4 arguments (including the program name)
    if args.len() == 4 {
        let source_url = args[2].clone();
        let destination_url = args[3].clone();
        match command.as_str() {
            "cp" => return Command::Copy(username, password, source_url, destination_url),
            "mv" => return Command::Move(username, password, source_url, destination_url),

            _ => {
                println!("Invalud command: {command}");
                std::process::exit(1);
            }
        }
    }

    let server_url = args[2].clone();
    match command.as_str() {
        "ls" => Command::List(username, password, server_url),
        "mkdir" => Command::MakeDir(username, password, server_url),
        "rm" => Command::Remove(username, password, server_url),
        "rmdir" => Command::RemoveDir(username, password, server_url),

        _ => {
            println!("Invalud command: {command}");
            std::process::exit(1);
        }
    }
}


fn get_username(url: &str) -> Result<String, Box<dyn Error>> {
    match Url::parse(url) {
        Ok(parsed_url) => {
            let username = parsed_url.username();
            if username != "" {
                    return Ok(username.to_string());
            }
        }
        Err(error) => {
            eprintln!("Error parsing url: {:?}", error);
        }
    }
    Ok("".to_string())
}

fn get_password(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    match Url::parse(url) {
        Ok(parsed_url) => {
            
            if let Some(password) = parsed_url.password() {
                let _password = password.to_string();
                    return Ok(Some(_password));
            }
        }
        Err(error) => {
            eprintln!("Error parsing url: {:?}", error);
        }
    }
    Ok(None)
}