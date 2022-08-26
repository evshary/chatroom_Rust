use std::env;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_std::prelude::*;
use futures::channel::mpsc;
use std::sync::Arc;
use std::collections::HashMap;

const MSG_SIZE: usize = 100;

enum Msg {
    New {
        sender: Arc<Box<String>>,
        stream: TcpStream,
    },
    Close {
        sender: Arc<Box<String>>,
    },
    Data {
        sender: Arc<Box<String>>,
        payload: String,
    },
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
    let (tx, mut rx) = mpsc::unbounded::<Msg>();
    task::spawn(async move {
        let mut hash = HashMap::new();
        loop {
            let event = rx.next().await.unwrap();
            match event {
                Msg::New { sender, stream } => {
                    hash.insert(sender, stream);
                },
                Msg::Data { sender, payload } => {
                    for (name, mut stream) in &hash {
                        if *name != sender {
                            stream.write_all(payload.as_bytes()).await.unwrap();
                        }
                    }
                },
                Msg::Close { sender } => {
                    hash.remove(&sender);
                },
            }
        }
    });

    while let Some(stream) = listener.incoming().next().await {
        let mut stream = stream.unwrap();
        let tx = tx.clone();
        task::spawn(async move {
            let name = Arc::new(Box::new(format!("{}", stream.peer_addr().unwrap())));
            println!("Connected by {}", name);
            tx.unbounded_send(Msg::New { sender: name.clone(), stream: stream.clone() }).unwrap();
            let mut buf = [0; MSG_SIZE];
            loop {
                let size = stream.read(&mut buf).await.unwrap();
                if size != 0 {
                    print!("{}", String::from_utf8(buf[..size].to_vec()).unwrap());
                    tx.unbounded_send(Msg::Data { sender: name.clone(), payload: String::from_utf8(buf[..size].to_vec()).unwrap() }).unwrap();
                } else {
                    println!("{} is disconnected", name);
                    tx.unbounded_send(Msg::Close { sender: name.clone() }).unwrap();
                    break;
                }
            }
        });
    }
}