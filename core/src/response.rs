use super::game::{board::Board, piece::Piece, state::GameState};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Valid { board: Board, state: GameState },
    Invalid,
    Init { board: Board, piece: Piece },
    Connect,
    Disconnect,
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Response as R;
        let s = match self {
            R::Valid { .. } => "Valid",
            R::Invalid => "Invalid",
            R::Init { .. } => "Init",
            R::Connect => "The other player connected",
            R::Disconnect => "The other player disconnected",
        };

        write!(f, "{s}")
    }
}
