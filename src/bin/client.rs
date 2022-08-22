use std::env;

fn main() {
    println!("Chatroom client:");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: client <IP> <port>");
        return;
    }
    let ip = &args[1];
    let port: u32 = args[2].parse().unwrap();
    println!("Connect to {}:{}", ip, port);
}