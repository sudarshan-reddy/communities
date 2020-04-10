use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Result;
use std::rc::Rc;

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
    fn new() -> Self;
    fn broadcast(&self, message: Message) -> Result<()>;
    fn add(&mut self, id: &str) -> Result<()>;
    fn remove(&mut self, id: &str) -> Result<()>;
}

pub struct SimpleClient {}

impl Client for SimpleClient {
    fn send(&self, message: Message) -> Result<()> {
        println!("send called");
        Ok(())
    }

    fn receive(&self) -> Result<Message> {
        println!("receive called");
        Ok((Message {
            id: "abcd",
            body: "test",
        }))
    }
}

pub struct MemoryStore<Cl: Client> {
    clients: Rc<RefCell<HashMap<String, Cl>>>,
}

impl<Cl> ClientStore for MemoryStore<Cl>
where
    Cl: Client,
{
    fn new() -> Self {
        MemoryStore {
            clients: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn broadcast(&self, message: Message) -> Result<()> {
        Ok(())
    }

    fn add(&mut self, id: &str) -> Result<()> {
        Ok(())
    }

    fn remove(&mut self, id: &str) -> Result<()> {
        Ok(())
    }
}
