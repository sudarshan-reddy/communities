use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const MSG_SIZE: usize = 32;

pub struct Room<'a, T> {
    clients: Vec<Client<'a, T>>,
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl<'a, T> Room<'a, T> {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Room {
            clients: vec![],
            tx: tx,
            rx: rx,
        }
    }

    fn newClient(&self, stream: TcpStream) -> Client<T> {
        let client = Client::new(stream, self.tx.clone(), &self.rx);
        client
    }
}

#[derive(Debug)]
pub struct Client<'a, T> {
    socket: TcpStream,
    tx: Sender<T>,
    rx: &'a Receiver<T>,
}

impl<'a, T> Client<'a, T> {
    pub fn new(stream: TcpStream, tx: Sender<T>, rx: &'a Receiver<T>) -> Self {
        Client {
            socket: stream,
            tx: tx,
            rx: rx,
        }
    }

    pub fn send(&mut self, msg: T) -> Result<(), ClientError<T>> {
        let mut buf = vec![0; MSG_SIZE];
        match self.socket.read(&mut buf) {
            Ok(_) => {
                //let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                //let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                self.tx.send(msg)?;
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(e) => {
                println!("err: {}", e);
            }
        }
        Ok(())
    }

    pub fn receive(&self) -> Result<T, ClientError<T>> {
        let res = self.rx.try_recv()?;
        Ok(res)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClientError<T> {
    Recv(mpsc::TryRecvError),
    Send(mpsc::SendError<T>),
}

impl<T> fmt::Display for ClientError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientError::Recv(ref err) => err.fmt(f),
            ClientError::Send(ref err) => err.fmt(f),
        }
    }
}

impl<T> From<mpsc::TryRecvError> for ClientError<T> {
    fn from(err: mpsc::TryRecvError) -> ClientError<T> {
        ClientError::Recv(err)
    }
}

impl<T> From<mpsc::SendError<T>> for ClientError<T> {
    fn from(err: mpsc::SendError<T>) -> ClientError<T> {
        ClientError::Send(err)
    }
}
