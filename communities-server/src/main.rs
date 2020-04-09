use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

mod room;

fn main() {
    start_server().unwrap();
}

fn start_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142")?;

    for stream in listener.incoming() {
        //TODO: inject handler through startServer.
        let res = stream?;
        thread::spawn(move || handle_client(res));
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                println!("echoed: {:?}\n", &data[0..size]);
            }
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
