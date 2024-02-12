use super::{piece::Piece, state::GameState};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Board([Option<Piece>; 9]);

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_full(&self) -> bool {
        self.0.iter().flatten().count() == 9
    }

    pub fn data(&self) -> &[Option<Piece>] {
        &self.0
    }

    pub fn check_end(&self, piece: Piece) -> GameState {
        for i in 0..3 {
            if self[(i, 0)].is_some()
                && self[(i, 0)] == self[(i, 1)]
                && self[(i, 1)] == self[(i, 2)]
            {
                return GameState::Win(piece);
            }

            if self[(0, i)].is_some()
                && self[(0, i)] == self[(1, i)]
                && self[(1, i)] == self[(2, i)]
            {
                return GameState::Win(piece);
            }
        }

        if self[(0, 0)].is_some() && self[(0, 0)] == self[(1, 1)] && self[(1, 1)] == self[(2, 2)] {
            return GameState::Win(piece);
        }

        if self[(0, 2)].is_some() && self[(0, 2)] == self[(1, 1)] && self[(1, 1)] == self[(2, 0)] {
            return GameState::Win(piece);
        }

        if self.is_full() {
            return GameState::Stalemate;
        }

        GameState::Playing
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Option<Piece>;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.0[i * 3 + j]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.0[i * 3 + j]
    }
}

impl FromStr for Board {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = [None; 9];

        s.split_whitespace()
            .take(9)
            .map(|c| Piece::from_str(c).ok())
            .enumerate()
            .for_each(|(i, p)| pieces[i] = p);

        Ok(Self(pieces))
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pieces: Vec<String> = self
            .0
            .iter()
            .map(|x| x.map(|p| p.to_string()).unwrap_or(" ".to_string()))
            .collect();

        let mut iter = pieces.chunks(3);
        let v1 = iter.next().unwrap();
        let v2 = iter.next().unwrap();
        let v3 = iter.next().unwrap();

        let mut s = String::new();
        s.push_str(&format!(" {} | {} | {} \n", v1[0], v1[1], v1[2]));
        s.push_str(" - + - + - \n");
        s.push_str(&format!(" {} | {} | {} \n", v2[0], v2[1], v2[2]));
        s.push_str(" - + - + - \n");
        s.push_str(&format!(" {} | {} | {} \n", v3[0], v3[1], v3[2]));

        write!(f, "{s}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        assert!(Board::from_str("x o o - - - o o x").is_ok());
        assert!(Board::from_str("o - o - o - o - o").is_ok());

        let board = Board::from_str("1 2 3 4 5 6 7 8").unwrap();
        assert!(board.0.iter().all(|p| p.is_none()));

        let board = Board::from_str("1 2 3 4 5 6 7 8 9 10").unwrap();
        assert!(board.0.iter().all(|p| p.is_none()));

        let board = Board::from_str("o o o o o o o o o o").unwrap();
        assert!(board.0.iter().all(|p| p.is_some()));
    }

    #[test]
    fn end() {
        let board = Board::from_str("x x x  - - -  x x x").unwrap();
        assert_eq!(GameState::Win(Piece::X), board.check_end(Piece::X));

        let board = Board::from_str("- - -  x x x  - - -").unwrap();
        assert_eq!(GameState::Win(Piece::X), board.check_end(Piece::X));

        let board = Board::from_str("- - -  - - -  x x x").unwrap();
        assert_eq!(GameState::Win(Piece::X), board.check_end(Piece::X));

        let board = Board::from_str("x - -  - x -  - - x").unwrap();
        assert_eq!(GameState::Win(Piece::X), board.check_end(Piece::X));

        let board = Board::from_str("- - x  - x -  x - -").unwrap();
        assert_eq!(GameState::Win(Piece::X), board.check_end(Piece::X));

        let board = Board::from_str("- - -  - - -  - - -").unwrap();
        assert_eq!(GameState::Playing, board.check_end(Piece::X));

        let board = Board::from_str("o x o  o x o  x o x").unwrap();
        assert_eq!(GameState::Stalemate, board.check_end(Piece::X));
    }
}
