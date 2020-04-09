use std::io::Result;

pub struct Room<CS: ClientStore> {
    clients: CS,
}

pub struct Message<'a> {
    pub id: &'a str,
    pub body: &'a str,
}

pub trait Client {
    fn send(&self, message: Message) -> Result<()>;
    fn receive(&self) -> Result<Message>;
}

pub trait ClientStore {
    fn new(client: dyn Client) -> Self;
    fn broadcast(&self, message: Message) -> Result<()>;
    fn add(&mut self, id: &str) -> Result<()>;
    fn remove(&mut self, id: &str) -> Result<()>;
}
