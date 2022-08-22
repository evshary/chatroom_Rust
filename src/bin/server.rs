use std::env;

fn main() {
    println!("Chatroom server:");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: server <port>");
        return;
    }
    let port: u32 = args[1].parse().unwrap();
    println!("Listening to port {}", port);
}