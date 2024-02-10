use super::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win(Piece),
    Stalemate,
}
