# FTP Client in Rust

## Overview

In this project, I ventured into the realm of Rust to build an FTP (File Transfer Protocol) client. This client is designed to connect to FTP servers, perform various file operations, and communicate with servers using FTP commands.

### High-Level Approach

Here's a quick overview of the approach I took:

1. **Parsing Command Line Arguments**: The client takes command-line arguments to figure out which FTP command to execute. Parsing these arguments extracts details like the FTP command, username, password, and URLs.

2. **Connecting to FTP Server**: The client establishes a connection with the FTP server specified in the URL, typically using the FTP control port (which is usually 21). It sends `USER` and `PASS` commands to authenticate.

3. **Handling FTP Commands**: Depending on the FTP command from the command line, the client sends the corresponding FTP command to the server. For example, it sends `LIST`, `MKD` (Make Directory), `RMD` (Remove Directory), `DELE` (Delete File), `STOR` (Store File), or `RETR` (Retrieve File) commands.

4. **Data Channel**: For data transfer operations, such as `STOR` and `RETR`, the client creates a data channel with the server. It calculates the new data port based on the server's response to the `PASV` command.

### Challenges

While working on this project, I encountered several challenges:

- **Learning Rust**: Rust was new to me, so I had to get accustomed to its syntax and conventions. 

- **Error Handling**: Rust places a strong emphasis on error handling. Ensuring that I handled errors correctly throughout the code, while keeping the code readable, was a learning experience.

### Testing

To test the code, I used responses from a local FTP server. I executed various FTP commands, such as listing directories, creating and removing directories, uploading and downloading files, and moving files.

