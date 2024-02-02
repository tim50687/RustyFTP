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
    let mut username = String::new();
    let mut password = String::new();
    
    // Check if there are exactly 4 arguments (including the program name)
    if args.len() == 4 {
        let mut source_url = String::new();
        let mut destination_url = String::new();
        // Check args[2] or args[3] is the host
        match get_host_str(&args[2]) {
            Ok(Some(_source)) => {
                source_url = _source;
                destination_url = args[3].clone();
                // Get username and password
                match get_username(&args[2]) {
                    Ok(_username) => {
                        if _username != "".to_string(){
                            username = _username;
                        }
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
            }
            _ => {
                match get_host_str(&args[3]) {
                    Ok(Some(_destination)) => {
                        source_url = args[2].clone();
                        destination_url = _destination;
                        // Get username and password
                        match get_username(&args[3]) {
                            Ok(_username) => {
                                if _username != "".to_string(){
                                    username = _username;
                                }
                            }
                            _ => {
                                username = "".to_string();
                            }
                        }
                        match get_password(&args[3]) {
                            Ok(Some(_password)) => {
                                password = _password;
                            }
                            _ => {
                                password = "".to_string();
                            }
                        }
                    }
                    _ => {

                    }
                }
            }
        }

        match command.as_str() {
            "cp" => return Command::Copy(username, password, source_url, destination_url),
            "mv" => return Command::Move(username, password, source_url, destination_url),

            _ => {
                println!("Invalud command: {command}");
                std::process::exit(1);
            }
        }
    }

    // 3 arguments
    let mut host_str = String::new();
    match get_host_str(&args[2]) {
        Ok(Some(_host_str)) => {
            host_str = _host_str;
        }
        _ => {
            host_str = "".to_string();
        }
    }
    // Get username and password
    match get_username(&args[2]) {
        Ok(_username) => {
            if _username != "".to_string(){
                username = _username;
            }
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
    match command.as_str() {
        "ls" => Command::List(username, password, host_str),
        "mkdir" => Command::MakeDir(username, password, host_str),
        "rm" => Command::Remove(username, password, host_str),
        "rmdir" => Command::RemoveDir(username, password, host_str),

        _ => {
            println!("Invalud command: {command}");
            std::process::exit(1);
        }
    }
}

fn get_host_str(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    match Url::parse(url) {
        Ok(parsed_url) => {

                return Ok(Some(parsed_url.path().to_string()));
            
        }
        Err(error) => {
            eprintln!("Error parsing url: {:?}", error);
        }
    }
    Ok(None)
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


// Extract last two number from the ftp server when creating data channel
pub fn extract_last_two_numbers(response: &str) -> Option<(u16, u16)>{
    let parts: Vec<&str> = response.split(',').collect();

    let last: Vec<&str> = parts[parts.len() - 1].split(')').collect();

    if let(Ok(second_last), Ok(last)) = (parts[parts.len() - 2].parse::<u16>(),last[last.len() - 2].parse::<u16>()) {
        return Some((second_last, last));
    }
    None
}