mod utils;
mod ftp;

use std::env;
use utils::{Command, parse_arguments};
use ftp::{connect_to_ftp_server};


fn main() {
    // Connect to ftp server and reverive hello
    // let host: &str = "ftp.4700.network";
    // let port: &str = "21";
    // let _stream = connect_to_ftp_server(host, port);

    // Parse arguments
    let args: Vec<String>= env::args().collect(); 
    let command = parse_arguments(&args);
    println!("Command is {:?}", command);
}


