mod utils;
mod ftp;

use std::env;
use utils::{Command, parse_arguments};
use ftp::{connect_to_ftp_server, send_username_command, send_user_password_command};


fn main() {
    // Connect to ftp server and reverive hello
    let host: &str = "ftp.4700.network";
    let port: &str = "21";
    match connect_to_ftp_server(host, port) {

        Ok(mut _stream) => {
            // Parse arguments
            let args: Vec<String>= env::args().collect(); 
            let command = parse_arguments(&args);
            println!("Command is {:?}", command);
        
            match command {
                Command::List(username, password, url) => {
                    if let Ok(message) =  send_username_command(&mut _stream, &username) {
                        println!("{}", message);
                    }
                    if let Ok(message) = send_user_password_command(&mut _stream, &password) {
                        println!("{}", message);
                    }
                }
                _ => {
                    println!("error");
                }
                // Command::MakeDir(username, password, url) {
        
                // }
                // Command::Remove(username, password, url) {
        
                // }
                // Command::RemoveDir(username, password, url) {
        
                // }
                // Command::List(username, password, source, destination) {
        
                // }Command::List(username, password, source, destination) {
        
                // }
                
            }

        }
        Err(err) =>
        {
            eprintln!("Error connecting to FTP server: {:?}", err);
        }


    }

    // loop {
        
    // }
}


