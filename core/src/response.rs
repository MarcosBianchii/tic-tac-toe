use super::game::board::Board;
use super::game::piece::Piece;
use super::game::state::GameState;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Valid { board: Board, state: GameState },
    Invalid,
    WaitingForPlayer,
    Connect { board: Board, piece: Piece },
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Response as R;
        let s = match self {
            R::Valid { .. } => "Valid",
            R::Invalid => "Invalid",
            R::WaitingForPlayer => "Waiting for player...",
            R::Connect { .. } => "Connected",
        };

        write!(f, "{s}")
    }
}
