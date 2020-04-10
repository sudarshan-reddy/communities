use std::collections::HashMap;
use std::io::Result;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub struct Room<CS: ClientStore> {
    clients: CS,
}

#[derive(Debug, Clone, Copy)]
pub struct Message<'a> {
    pub id: &'a str,
    pub body: &'a str,
}

pub trait Client {
    fn send(&mut self, message: Message) -> Result<()>;
    fn receive(&mut self) -> Result<Message>;
}

pub trait ClientStore {
    fn new() -> Self;
    fn broadcast(&mut self, message: Message) -> Result<()>;
    fn add(&mut self, id: &str, client: Box<dyn Client>) -> Result<()>;
    fn remove(&mut self, id: &str) -> Result<()>;
}

pub struct TCPClient {
    stream: TcpStream,
}

impl TCPClient {
    pub fn new(stream: TcpStream) -> Self {
        TCPClient { stream: stream }
    }
}

impl Client for TCPClient {
    fn send(&mut self, message: Message) -> Result<()> {
        println!("{:?}", message);
        self.stream.write(message.body.as_bytes())?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Message> {
        let mut data = [0; 50];
        while match self.stream.read(&mut data) {
            Ok(size) => {
                if size > 0 {
                    println!("echoed: {:?}\n", &data[0..size]);
                }
                true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    self.stream.peer_addr().unwrap()
                );
                self.stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}

        Ok(Message {
            id: "abcd",
            body: "test",
        })
    }
}

pub struct MemoryStore {
    clients: HashMap<String, Box<dyn Client>>,
}

impl ClientStore for MemoryStore {
    fn new() -> Self {
        MemoryStore {
            clients: HashMap::new(),
        }
    }

    fn broadcast(&mut self, message: Message) -> Result<()> {
        for v in self.clients.values_mut() {
            v.send(message)?;
        }
        Ok(())
    }

    fn add(&mut self, id: &str, client: Box<dyn Client>) -> Result<()> {
        self.clients.insert(id.to_string(), client);
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<()> {
        self.clients.remove(id);
        Ok(())
    }
}
