use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6142";
const MSG_SIZE: usize = 32;

fn main() {
    let listener = TcpListener::bind(LOCAL).expect("bind failed");
    listener.set_nonblocking(true).unwrap();

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    for (mut socket, addr) in listener.accept() {
        println!("client: {} connected...", addr);

        let tx = tx.clone();
        clients.push(socket.try_clone().expect("failed to clone client"));

        thread::spawn(move || loop {
            let mut buf = vec![0; MSG_SIZE];
            match socket.read_exact(&mut buf) {
                Ok(_) => {}
                Err(e) => {
                    println!("closing connection with: {} , err: {}", addr, e);
                    break;
                }
            }
        });
    }

    for msg in rx.try_recv() {
        clients = clients
            .into_iter()
            .filter_map(|mut client| {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);

                client.write_all(&buf).map(|_| client).ok()
            })
            .collect::<Vec<_>>();
    }
}
