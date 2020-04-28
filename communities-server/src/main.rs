mod server;

mod codec;

mod lib;

fn main() {
    let mut server = server::Server::new("0.0.0.0:1234");

    server.run().unwrap();
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;
