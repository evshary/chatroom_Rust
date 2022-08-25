use std::env;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_std::prelude::*;
use std::sync::{Arc, Mutex};

const MSG_SIZE: usize = 100;
const CHAT_SIZE: usize = 20;

struct Chatter {
    stream: Option<TcpStream>,
}

#[async_std::main]
async fn main() {
    println!("Chatroom server:");

    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: server <port>");
        return;
    }
    let port: u32 = args[1].parse().unwrap();
    println!("Listening to port {}", port);

    // Setup server listener
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    let senders = Arc::new(Mutex::new(vec![]));
    for _ in 0..CHAT_SIZE {
        senders.lock().unwrap().push(Chatter{stream: None});
    }

    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream.unwrap();
        let mut slot = 0;
        for (idx, s) in senders.lock().unwrap().iter_mut().enumerate() {
            if (*s).stream.is_none() {
                (*s).stream = Some(stream.clone());
                slot = idx;
                break;
            }
        }
        let senders = senders.clone();
        task::spawn(async move {
            println!("Connected by {}", stream.peer_addr().unwrap());
            let mut buf = [0; MSG_SIZE];
            loop {
                let size = stream.read(&mut buf).await.unwrap();
                if size != 0 {
                    println!("{}", String::from_utf8(buf[..size].to_vec()).unwrap());
                    for (idx, s) in senders.lock().unwrap().iter_mut().enumerate() {}
                } else {
                    println!("Disconnected");
                    senders.lock().unwrap()[slot].stream = None;
                    break;
                }
            }
        });
    }
}