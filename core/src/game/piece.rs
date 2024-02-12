use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    pub fn next(&mut self) {
        *self = match *self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        };
    }

    pub fn other(&self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self::X
    }
}

impl FromStr for Piece {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "X" => Ok(Self::X),
            "O" => Ok(Self::O),
            _ => Err("Invalid Piece representation"),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Piece::X => 'X',
            Piece::O => 'O',
        };

        write!(f, "{s}")
    }
}
