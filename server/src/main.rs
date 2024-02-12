mod game;
mod server;
mod threadpool;
use server::Server;
use std::env;

fn main() -> Result<(), &'static str> {
    let mut args = env::args().skip(1);

    let port = args
        .next()
        .ok_or("Usage: server <port>")?
        .parse::<u16>()
        .map_err(|_| "Invalid port number")?;

    let sv = Server::new([127, 0, 0, 1], port);
    sv.run(2)
}
