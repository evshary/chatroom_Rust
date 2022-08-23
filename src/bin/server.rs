use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::io::{self, Read, Write};

const MSG_SIZE: usize = 100;
const CHAT_SIZE: usize = 20;

struct Chatter {
    stream: Option<TcpStream>,
}

struct Msg {
    data: String,
    sender: usize,
}

fn main() {
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
    let listener = TcpListener::bind(addr).unwrap();
    let (tx, rx) = mpsc::channel::<Msg>();
    let senders = Arc::new(Mutex::new(vec![]));
    for _ in 0..CHAT_SIZE {
        senders.lock().unwrap().push(Chatter{stream: None});
    }

    // Create sender thread
    {
        let senders = senders.clone();
        thread::spawn(move || loop {
            let msg = rx.recv().unwrap();
            print!("{}", msg.data);
            io::stdout().flush().unwrap();  // Need to flush the stdout
            for (idx, s) in senders.lock().unwrap().iter_mut().enumerate() {
                if idx != msg.sender && s.stream.is_some() {
                    s.stream.as_ref().unwrap().write_all(msg.data.as_bytes()).expect("Unable to send msg");
                }
            }
        });
    }

    // Recv connection from client
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut slot = 0;
                for (idx, s) in senders.lock().unwrap().iter_mut().enumerate() {
                    if (*s).stream.is_none() {
                        (*s).stream = Some(stream.try_clone().unwrap());
                        slot = idx;
                        break;
                    }
                }
                println!("Connected by {}", stream.peer_addr().unwrap());
                let tx = tx.clone();
                {
                    let senders = senders.clone();
                    thread::spawn(move || {
                        let mut buf = [0; MSG_SIZE];
                        let slot = slot;
                        loop {
                            let size = stream.read(&mut buf).expect("Receiving data error.");
                            if size != 0 {
                                let msg = Msg {
                                    data: String::from_utf8(buf[..size].to_vec()).expect("Invalid message"),
                                    sender: slot,
                                };
                                tx.send(msg).expect("Internal error");
                            } else {
                                println!("Disconnected");
                                senders.lock().unwrap()[slot].stream = None;
                                break;
                            }
                        }
                    });
                }
            }
            Err(e) => {
                println!("Error connection: {}", e);
            }
        }
    }
}