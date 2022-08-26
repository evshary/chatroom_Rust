use std::env;
use std::str;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::select;
use futures::FutureExt; // fuse()
use async_std::io;

const MSG_SIZE: usize = 100;

#[async_std::main]
async fn main() {
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
    let mut stream = TcpStream::connect(addr).await.unwrap();

    // Recv data from stdin and server
    let mut buf = [0; MSG_SIZE];
    let mut line = String::new();
    let stdin = io::stdin();
    loop {
        select! {
            size = stream.read(&mut buf).fuse() => {
                let size = size.unwrap();
                let msg = str::from_utf8(&buf[..size]).unwrap();
                print!("{}", msg);
            },
            _ = stdin.read_line(&mut line).fuse() => {
                stream.write_all(line.as_bytes()).await.unwrap();
            },
        }
    };
}