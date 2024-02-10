use super::game::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Valid { piece: Piece, idx: (usize, usize) },
    Invalid,
    WaitingForPlayer,
    Connect { piece: Piece },
}
