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
        use GameState as Gs;
        matches!(self, Gs::Win(_) | Gs::Stalemate)
    }
}
