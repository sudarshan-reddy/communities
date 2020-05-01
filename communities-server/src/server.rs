use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    rx: Receiver<Action>,
    tx: Sender<Action>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InputMessage {
    id: String,
    // TODO: change this out with an enum.
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerMessage {
    sender: String,
    msg: String,
    error: bool,
}

#[derive(Debug)]
struct Action {
    msg_type: MsgType,
    msg: String,
    sender: String,
}

#[derive(Debug, Clone, Copy)]
enum MsgType {
    Error,
    Broadcast,
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
                thread::spawn(move || {
                    handle_client(&socket, &tx).unwrap();
                });
            }

            self.broadcast();
        }
    }

    fn broadcast(&self) {
        if let Ok(msg) = self.rx.try_recv() {
            println!("recv: {:?}", msg);
            for client in &self.clients {
                serde_json::to_writer(
                    client.clone(),
                    &ServerMessage {
                        sender: msg.sender.clone(),
                        msg: msg.msg.clone(),
                        error: false,
                    },
                )
                .unwrap();
            }
        }
    }
}

fn handle_client(stream: &TcpStream, tx: &Sender<Action>) -> crate::Result<()> {
    let client_request: InputMessage = match serde_json::from_reader(stream.try_clone().unwrap()) {
        Ok(v) => v,
        Err(e) => {
            let action = Action {
                msg_type: MsgType::Error,
                msg: e.to_string(),
                sender: "server".to_string(),
            };
            println!("error: {:?}", action);
            tx.send(action)?;
            return Err(Box::new(e));
        }
    };

    let action = Action {
        msg_type: MsgType::Broadcast,
        msg: client_request.msg.clone(),
        sender: client_request.id.clone(),
    };

    println!("msg: {:?}", action);
    tx.send(action).unwrap();
    Ok(())
}
