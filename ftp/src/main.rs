// Import modules
mod utils;
mod ftp;

// Import necessary standard library modules
use std::env;

// Import functions and structs from modules
use utils::{Command, parse_arguments, extract_last_two_numbers};
use ftp::{
    connect_to_ftp_server, init_download_upload, send_quit_command, send_mkdir_command, 
    send_rmdir_command, send_pasv_command, login, send_list_command, send_copy_command, 
    send_move_command, send_remove_command};


fn main() {
    // Connect to ftp server and reverive hello
    let host: &str = "ftp.4700.network";
    let port: &str = "21";
    // Connect to the FTP server and handle potential errors
    match connect_to_ftp_server(host, port) {
        Ok(mut _stream) => {
            // Parse arguments
            let args: Vec<String>= env::args().collect(); 
            let command = parse_arguments(&args);
        
            // Handle different FTP commands based on parsed arguments
            match command {
                // Handle the MakeDir command
                Command::MakeDir(username, password, url) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Send the MKD command and print the response
                    if let Ok(message) = send_mkdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                // Handle the RemoveDir command
                Command::RemoveDir(username, password, url) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Send the RMD command and print the response
                    if let Ok(message) = send_rmdir_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                // Handle the Remove command
                Command::Remove(username, password, url) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Send the DELE command and print the response
                    if let Ok(message) = send_remove_command(&mut _stream, &url) {
                        println!("{}", message);
                    }
                }
                // Handle the List command
                Command::List(username, password, url) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate the new data port
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }

                    // Connect to the FTP server's data channel
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE, MODE, and STRU settings
                            init_download_upload(&mut _stream);

                            // Send the LIST command and print the response
                            if let Ok(message) =
                                send_list_command(&mut _stream, &mut data_stream, &url)
                            {
                                println!("{}", message);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }
                // Handle the Copy command
                Command::Copy(username, password, source, destination) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate the new data port
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }

                    // Connect to the FTP server's data channel
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE, MODE, and STRU settings
                            init_download_upload(&mut _stream);

                            // Send the COPY command and print the response
                            if let Ok(message) = send_copy_command(
                                &mut _stream,
                                &mut data_stream,
                                &source,
                                &destination,
                                &args,
                            ) {
                                println!("{}", message);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }
                // Handle the Move command
                Command::Move(username, password, source, destination) => {
                    // Login to the FTP server
                    login(&mut _stream, &username, &password);

                    // Set up data channel
                    let mut new_port = String::new();
                    if let Ok(message) = send_pasv_command(&mut _stream) {
                        if let Some((second_last, last)) = extract_last_two_numbers(&message) {
                            // Calculate the new data port
                            new_port = ((second_last << 8) + last).to_string();
                        }
                    }

                    // Connect to the FTP server's data channel
                    match connect_to_ftp_server(host, &new_port) {
                        Ok(mut data_stream) => {
                            // Turn on TYPE, MODE, and STRU settings
                            init_download_upload(&mut _stream);

                            // Send the MOVE command and print the response
                            if let Ok(message) = send_move_command(
                                &mut _stream,
                                &mut data_stream,
                                &source,
                                &destination,
                                &args,
                            ) {
                                println!("{}", message);
                            }
                        }
                        Err(err) => {
                            eprintln!("Error creating data channel: {:?}", err);
                        }
                    }
                }
                // Handle other unknown commands
                _ => {
                    println!("Error: Unknown command");
                }
            }
            // Send the QUIT command to close the FTP session
            if let Ok(message) = send_quit_command(&mut _stream) {
                println!("{}", message);
            }
        }
        Err(err) => {
            eprintln!("Error connecting to FTP server: {:?}", err);
        }
    }
}