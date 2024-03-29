use core::game::{board::Board, piece::Piece};
use core::response::Response;
use core::{io_err, write_str};
use std::collections::BTreeMap;
use std::io;
use std::net::TcpStream;

pub struct Game {
    pub board: Board,
    pub players: BTreeMap<Piece, TcpStream>,
    turn: Piece,
    started: Piece,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    fn alert_other_player(&mut self, piece: Piece) -> io::Result<()> {
        if let Some(other) = self.players.get_mut(&piece.other()) {
            let res = Response::Connect;
            let json = serde_json::to_string(&res)?;
            write_str(other, &json)?;
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

            self.alert_other_player(piece).ok()?;
            self.players.insert(piece, stream);
            piece
        })
    }

    pub fn disconnect(&mut self, piece: Piece) -> io::Result<()> {
        self.broadcast(Response::Disconnect(piece))?;
        self.players.remove(&piece);
        Ok(())
    }

    pub fn play(&mut self, piece: Piece, idx: (usize, usize)) -> Response {
        if piece != self.turn {
            return Response::Invalid(String::from("Not your turn"));
        }

        if self.board[idx].is_some() {
            return Response::Invalid(String::from("Cell already occupied"));
        }

        self.board[idx] = Some(piece);
        self.turn.next();

        let state = self.board.check_end(piece);

        if state.is_end() {
            self.turn = self.started.other();
            self.started = self.turn;
            self.board.clear();
        }

        Response::Valid { piece, idx, state }
    }

    pub fn send(&mut self, piece: Piece, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;

        self.players
            .get_mut(&piece)
            .map(|stream| write_str(stream, &json))
            .ok_or(io_err!("Failed to send reponse"))?
    }

    pub fn broadcast(&mut self, res: Response) -> io::Result<()> {
        let json = serde_json::to_string(&res)?;

        for stream in self.players.values_mut() {
            write_str(stream, &json)?;
        }

        Ok(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::new(),
            players: BTreeMap::new(),
            turn: Piece::default(),
            started: Piece::default(),
        }
    }
}
