use crate::game::Game;
use crate::threadpool::ThreadPool;
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

    pub fn send<W: Write>(stream: &mut W, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;
        write_str(stream, &json)
    }

    fn handle_client(mut stream: TcpStream, game: Arc<Mutex<Game>>) -> io::Result<()> {
        let piece = game.lock().unwrap().assign_piece(stream.try_clone()?);
        Self::send(&mut stream, Response::Connect { piece })?;
        let ip = stream.peer_addr().unwrap();
        println!("player {piece} ({ip}) connected");

        loop {
            let Ok(data) = read_str(&mut stream) else {
                return Ok(());
            };

            // Get Request from client.
            let mut game = game.lock().unwrap();
            let idx = match serde_json::from_str(&data) {
                Ok(Request::Play { idx }) => idx,
                Ok(Request::Disconnect) => {
                    let ip = stream.local_addr().unwrap();
                    println!("player {piece} ({ip}) disconnected");
                    game.players.remove(&piece);
                    return Ok(());
                }

                Err(_) => {
                    let _ = Self::send(&mut stream, Response::Invalid);
                    continue;
                }
            };

            // Check if the game is full.
            if game.players.len() < 2 {
                let _ = Self::send(&mut stream, Response::WaitingForPlayer);
                continue;
            }

            // Check if the piece is valid.
            if !game.players.contains_key(&piece) {
                let _ = Self::send(&mut stream, Response::Invalid);
                continue;
            }

            // Check if it's the player's turn.
            if piece != game.turn {
                let _ = Self::send(&mut stream, Response::Invalid);
                continue;
            }

            // Check if the position is empty.
            if game.board[idx].is_some() {
                let _ = Self::send(&mut stream, Response::Invalid);
                continue;
            }

            // Update the board and broadcast the response.
            game.board[idx] = Some(piece);
            println!("{:?}", game.board);

            game.turn.next();
            game.broadcast(Response::Valid { piece, idx })?;
            drop(game);
        }
    }

    pub fn run(self, nthreads: usize) -> Result<(), &'static str> {
        let Ok(listener) = TcpListener::bind(self.0) else {
            return Err("Failed to bind to address");
        };

        let game = Arc::new(Mutex::new(Game::new()));
        let pool = ThreadPool::new(nthreads);
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
