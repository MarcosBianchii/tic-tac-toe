use core::game::state::GameState;
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

    pub fn assign_piece(&mut self, stream: TcpStream) -> Option<Piece> {
        (self.players.len() < 2).then_some({
            let piece = if self.players.contains_key(&Piece::X) {
                Piece::O
            } else {
                Piece::X
            };

            self.players.insert(piece, stream);
            piece
        })
    }

    pub fn play(&mut self, piece: Piece, idx: (usize, usize)) -> Response {
        if piece != self.turn {
            return Response::Invalid;
        }

        if self.board[idx].is_some() {
            return Response::Invalid;
        }

        self.board[idx] = Some(piece);
        self.turn.next();

        let state = self.board.check_end(piece);
        let board = self.board;

        if state != GameState::Playing {
            self.board = Board::new();
            self.turn = Piece::default();
        }

        Response::Valid { board, state }
    }

    pub fn send(&mut self, piece: Piece, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;

        self.players
            .get_mut(&piece)
            .map(|stream| write_str(stream, &json))
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to send response",
            ))?
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
