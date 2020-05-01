use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    rx: Receiver<InputMessage>,
    tx: Sender<InputMessage>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InputMessage {
    id: String,
    // TODO: change this out with an enum.
    action: String,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerMessage {
    sender: String,
    msg: String,
    error: bool,
}

struct action {
    msg_type: msgType,
}

enum msgType {
    error,
    broadcast,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let (tx, rx) = mpsc::channel();
        Server {
            listener: listener,
            clients: Vec::new(),
            rx: rx,
            tx: tx,
        }
    }

    pub fn run(&mut self) -> crate::Result<()> {
        loop {
            if let Ok((socket, addr)) = self.listener.accept() {
                let tx = self.tx.clone();
                self.clients.push(socket.try_clone()?);
                println!("connected to {:?}", addr);
                thread::spawn(move || loop {
                    handle_client(&socket, tx.clone());
                });
            }

            self.broadcast();
        }
    }

    fn broadcast(&self) {
        if let Ok(msg) = self.rx.try_recv() {
            self.clients.iter().map(|client| {
                serde_json::to_writer(
                    client.clone(),
                    &ServerMessage {
                        sender: msg.id.clone(),
                        msg: msg.msg.clone(),
                        error: false,
                    },
                )
                .unwrap();
            });
        }
    }
}

fn handle_client(stream: &TcpStream, tx: Sender<InputMessage>) -> crate::Result<()> {
    let client_request: InputMessage = match serde_json::from_reader(stream.try_clone().unwrap()) {
        Ok(v) => v,
        Err(e) => serde_json::to_writer(
            stream,
            &ServerMessage {
                sender: "server".to_string(),
                msg: e.to_string(),
                error: true,
            },
        )?,
    };
    tx.send(client_request).unwrap();
}
