use std::env;
use std::net::TcpStream;
use std::{thread, time};
use std::sync::mpsc;
use std::str;
use std::io::{self, Read, Write};

const MSG_SIZE: usize = 100;

fn main() {
    println!("Chatroom client:");

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: client <IP> <port>");
        return;
    }
    let ip = &args[1];
    let port: u32 = args[2].parse().unwrap();
    println!("Connect to {}:{}", ip, port);

    // Setup connection
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(addr).expect("Unable to connect");
    stream.set_nonblocking(true).expect("Unable to set nonblocking");
    println!("Connecting successfully");
    let (tx, rx) = mpsc::channel::<String>();

    // Create recv & send thread
    thread::spawn(move || {
        let mut buf = [0; MSG_SIZE];
        loop {
            // Read data from server
            match stream.read(&mut buf) {
                Ok(size) => {
                    if size != 0 {
                        let msg = str::from_utf8(&buf[..size]).unwrap();
                        print!("{}", msg);
                        io::stdout().flush().unwrap();  // Need to flush the stdout
                    }
                },
                Err(_) => {}
            }
            // Read data from user's input
            match rx.try_recv() {
                Ok(msg) => {
                    stream.write_all(msg.as_bytes()).expect("Unable to send message");
                },
                Err(_) => {}
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    });

    // Get user input
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Not correct input");
        tx.send(input).expect("Internal error");
    }
}