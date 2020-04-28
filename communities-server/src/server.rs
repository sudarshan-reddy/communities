use serde::{Deserialize, Serialize};
use std::io::BufWriter;
use std::net::{TcpListener, TcpStream};
use std::time::{self, Duration};

pub struct Server {
    listener: TcpListener,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputMessage {
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerMessage {
    msg: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        Server { listener: listener }
    }

    pub fn run(&mut self) -> crate::Result<()> {
        for socket in self.listener.incoming() {
            self.handle_client(socket?);
        }
        Ok(())
    }

    fn handle_client(&self, stream: TcpStream) {
        let client_request: InputMessage =
            match serde_json::from_reader(stream.try_clone().unwrap()) {
                Ok(v) => v,
                Err(e) => {
                    serde_json::to_writer(stream, &ServerMessage { msg: e.to_string() }).unwrap();
                    return;
                }
            };
        println!("{}", client_request.msg);
    }
}
