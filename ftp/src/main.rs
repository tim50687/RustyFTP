mod utils;
mod ftp;

use std::env;
use utils::{Command, parse_arguments};
use std::net::TcpStream;
use ftp::{connect_to_ftp_server, send_username_command, send_user_password_command, send_type_command, send_mode_command, send_stru_command, send_quit_command, send_mkdir_command, send_rmdir_command};

// Login username and password
fn login(stream: &mut TcpStream, username: &str, password: &str) -> () {
    if let Ok(message) =  send_username_command(stream, username) {
        println!("{}", message);
    }
    if let Ok(message) = send_user_password_command(stream, password) {
        println!("{}", message);
    }
}


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
                    login(&mut _stream, &username, &password);

                }
                Command::MakeDir(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    if let Ok(message) =  send_mkdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                Command::RemoveDir(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    println!("dadas");
                    if let Ok(message) =  send_rmdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                _ => {
                    println!("error");
                }
                
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


