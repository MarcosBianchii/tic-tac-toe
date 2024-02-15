use super::game::{board::Board, piece::Piece, state::GameState};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Valid {
        piece: Piece,
        idx: (usize, usize),
        state: GameState,
    },
    Invalid(String),
    Init {
        board: Board,
        piece: Piece,
    },
    Connect,
    Disconnect,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Response as R;
        let s = match self {
            R::Valid { .. } => "Valid move".to_string(),
            R::Invalid(msg) => format!("Invalid move: {msg}"),
            R::Init { .. } => "Init".to_string(),
            R::Connect => "The other player connected".to_string(),
            R::Disconnect => "The other player disconnected".to_string(),
        };

        write!(f, "{s}")
    }
}
