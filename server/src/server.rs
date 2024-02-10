use crate::game::Game;
use crate::threadpool::ThreadPool;
use core::game::piece::Piece;
use core::{read_str, write_str};
use core::{request::Request, response::Response};
use serde_json;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

pub struct Server(SocketAddr);

impl Server {
    pub fn new(address: [u8; 4], port: u16) -> Self {
        let [a, b, c, d] = address;
        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port));
        Self(address)
    }

    fn send<W: Write>(stream: &mut W, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;
        write_str(stream, &json)
    }

    fn register_user(stream: TcpStream, game: &Arc<Mutex<Game>>) -> Option<Piece> {
        game.lock()
            .map(|mut game| game.assign_piece(stream))
            .ok()
            .flatten()
    }

    fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>) -> io::Result<()> {
        let Some(piece) = Self::register_user(stream.try_clone()?, &game) else {
            return Ok(());
        };

        let ip = stream.peer_addr()?.ip();
        println!("Player `{piece}` ({ip}) connected");
        let board = game.lock().unwrap().board;
        Self::send(&mut stream, Response::Connect { piece, board })?;

        loop {
            println!("{:?}", game.lock().unwrap().board);

            let req = read_str(&mut stream)?;
            let req: Request = serde_json::from_str(&req)?;

            match req {
                Request::Play { idx } => {
                    let mut game = game.lock().unwrap();
                    let res = game.play(piece, idx);

                    match res {
                        Response::Valid { .. } => game.broadcast(res)?,
                        _ => game.send(piece, res)?,
                    }
                }

                Request::Disconnect => {
                    let mut game = game.lock().unwrap();
                    game.players.remove(&piece);
                    println!("Player `{piece}` ({ip}) disconnected");
                    break;
                }
            };
        }

        Ok(())
    }

    pub fn run(self, nthreads: usize) -> Result<(), &'static str> {
        let Ok(listener) = TcpListener::bind(self.0) else {
            return Err("Failed to bind to address");
        };

        let pool = ThreadPool::new(nthreads);
        let game = Arc::new(Mutex::new(Game::new()));
        println!("Listening at address {}", self.0);

        for stream in listener.incoming().flatten() {
            let game = Arc::clone(&game);
            pool.execute(move || {
                if let Err(e) = Self::handle_client(stream, game) {
                    eprintln!("{e}");
                }
            });
        }

        Ok(())
    }
}
