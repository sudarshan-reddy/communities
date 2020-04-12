use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:12233";
const MSG_SIZE: usize = 32;

fn main() {
    let listener = TcpListener::bind(LOCAL).expect("bind failed");
    listener.set_nonblocking(true).unwrap();

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("client: {} connected...", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buf = vec![0; MSG_SIZE];
                match socket.read(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send msg to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(e) => {
                        println!("{} , err: {}", addr, e);
                        break;
                    }
                }
                thread::sleep(::std::time::Duration::from_millis(100));
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buf = msg.clone().into_bytes();
                    buf.resize(MSG_SIZE, 0);

                    client.write_all(&buf).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        thread::sleep(::std::time::Duration::from_millis(100));
    }
}
