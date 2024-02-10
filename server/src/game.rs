use core::game::{board::Board, piece::Piece};
use core::response::Response;
use core::write_str;
use std::collections::BTreeMap;
use std::io;
use std::net::TcpStream;

pub struct Game {
    pub board: Board,
    pub players: BTreeMap<Piece, TcpStream>,
    pub turn: Piece,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn broadcast(&mut self, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;

        for (_, stream) in &mut self.players {
            write_str(stream, &json)?;
        }

        Ok(())
    }

    pub fn assign_piece(&mut self, stream: TcpStream) -> Piece {
        let piece = match self.players.len() {
            0 => Piece::X,
            1 => Piece::O,
            _ => unreachable!(),
        };

        self.players.insert(piece, stream);
        piece
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::new(),
            players: BTreeMap::new(),
            turn: Piece::default(),
        }
    }
}
