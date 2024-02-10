mod game;
mod server;
mod threadpool;
use server::Server;

fn main() -> Result<(), &'static str> {
    let sv = Server::new([127, 0, 0, 1], 8080);
    sv.run(2)
}
