use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
struct Client {
    id: String,
    tx: Sender<Message>,
}

pub enum Message {
    Message(String, String),
    Type(MessageType),
}

pub enum MessageType {
    Client,
    Server,
}

struct Room {
    // TODO: Make this threadsafe.
    connected_clients: RefCell<HashMap<SocketAddr, Client>>,
}

impl Room {
    fn new() -> Self {
        Room {
            connected_clients: RefCell::new(HashMap::new()),
        }
    }

    fn add(&self, addr: SocketAddr, client: Client) {
        self.connected_clients.borrow_mut().insert(addr, client);
    }

    fn remove(&self, addr: &SocketAddr) -> Option<Client> {
        self.connected_clients.borrow_mut().remove(addr)
    }
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    fn start(addr: &str) {
        let addr : String = addr.parse().unwrap();
        let mut listener = TcpListener::bind(&addr).unwrap();
        let server = async move {
            let mut incoming = listener.incoming();
            while let Some(socket_res) = incoming.next().await {
                match socket_res {
                    Ok(socket) => {
                        println!("Accepted connection from {:?}", socket.peer_addr());
                        // TODO: Process socket
                    } 
                    Err(err) => {
                        // Handle error by printing to STDOUT.
                        println!("accept error = {:?}", err);
                    }
                }
            }
        };
    }
}
