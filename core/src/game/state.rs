use super::piece::Piece;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GameState {
    Playing,
    Win(Piece),
    Stalemate,
}

impl GameState {
    pub fn is_end(&self) -> bool {
        matches!(self, GameState::Win(_) | GameState::Stalemate)
    }
}
