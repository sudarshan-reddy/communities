use std::error::Error;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

pub struct Room<T> {
    clients: Vec<Client<T>>,
}

pub struct Client<T> {
    socket: TcpStream,
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<T> Client<T> {
    pub fn new(stream: TcpStream) -> Self {
        let (tx, rx) = mpsc::channel();
        Client {
            socket: stream,
            tx: tx,
            rx: rx,
        }
    }

    pub fn send() {}

    pub fn receive(&self) -> Result<T, ClientReceiveError> {
        let res = self.rx.try_recv()?;
        Ok(res)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClientReceiveError {
    Recv(mpsc::TryRecvError),
}

impl fmt::Display for ClientReceiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientReceiveError::Recv(ref err) => err.fmt(f),
        }
    }
}

impl Error for ClientReceiveError {
    fn description(&self) -> &str {
        match *self {
            ClientReceiveError::Recv(ref err) => err.description(),
        }
    }
}

impl From<mpsc::TryRecvError> for ClientReceiveError {
    fn from(err: mpsc::TryRecvError) -> ClientReceiveError {
        ClientReceiveError::Recv(err)
    }
}
