mod utils;
mod ftp;

use std::env;
use utils::{Command, parse_arguments, extract_last_two_numbers};
use ftp::{connect_to_ftp_server, init_download_upload, send_quit_command, send_mkdir_command, send_rmdir_command, send_pasv_command, login, send_list_command, send_copy_command, send_move_command, send_remove_command};


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
                Command::MakeDir(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    if let Ok(message) = send_mkdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                Command::RemoveDir(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    if let Ok(message) = send_rmdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                Command::Remove(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    if let Ok(message) = send_remove_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                Command::List(username, password, url) => {
                    login(&mut _stream, &username, &password);
                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate new port 
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE MODE STRU
                            init_download_upload(&mut _stream);
                            if let Ok(message) = send_list_command(&mut _stream, &mut data_stream,&url) {
                                println!("{}", message);
                            }
                        }
                        Err(err) =>
                        {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }
                Command::Copy(username, password, source, destination) => {
                    login(&mut _stream, &username, &password);
                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate new port 
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE MODE STRU
                            init_download_upload(&mut _stream);
                            if let Ok(message) = send_copy_command(&mut _stream, &mut data_stream, &source, &destination, &args) {
                                println!("{}", message);
                            }
                        }
                        Err(err) =>
                        {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }   
                Command::Move(username, password, source, destination) => {
                    login(&mut _stream, &username, &password);
                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate new port 
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE MODE STRU
                            init_download_upload(&mut _stream);
                            if let Ok(message) = send_move_command(&mut _stream, &mut data_stream, &source, &destination, &args) {
                                println!("{}", message);
                            }
                        }
                        Err(err) =>
                        {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }
                _ => {
                    println!("error");
                }
            }
            // Quit the socket 
            if let Ok(message) =  send_quit_command(&mut _stream) {
                println!("{}", message);
            }
        }
        Err(err) =>
        {
            eprintln!("Error connecting to FTP server: {:?}", err);
        }
    }
}


