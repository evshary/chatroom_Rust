use std::env;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::io::{self, Read, Write};

const MSG_SIZE: usize = 100;

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
    let (tx, rx) = mpsc::channel::<String>();
    let senders = Arc::new(Mutex::new(vec![]));
    {
        let senders: Arc<Mutex<Vec<TcpStream>>> = senders.clone();
        thread::spawn(move || loop {
            let msg = rx.recv().unwrap();
            print!("{}", msg);
            io::stdout().flush().unwrap();  // Need to flush the stdout
            for s in senders.lock().unwrap().iter_mut() {
                s.write_all("abc123".as_bytes()).expect("Unable to write");
            }
        });
    }

    // Recv connection from client
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                senders.lock().unwrap().push(stream.try_clone().unwrap());
                println!("Connected by {}", stream.peer_addr().unwrap());
                let tx = tx.clone();
                thread::spawn(move || {
                    let mut buf = [0; MSG_SIZE];
                    loop {
                        match stream.read(&mut buf) {
                            Ok(size) => {
                                let msg = String::from_utf8(buf[..size].to_vec()).expect("Invalid message");
                                tx.send(msg).expect("Internal error");
                            },
                            Err(_) => {}
                        }
                    }
                });
            }
            Err(e) => {
                println!("Error connection: {}", e);
            }
        }
    }
}