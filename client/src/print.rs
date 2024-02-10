use colored::Colorize;
use core::game::{board::Board, piece::Piece};

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_board(board: &Board, idx: (usize, usize)) {
    for i in 0..3 {
        for j in 0..3 {
            print!(" ");
            let piece = if (i, j) == idx {
                "_".to_string().green()
            } else {
                board[(i, j)]
                    .map(|x| {
                        let s = x.to_string();
                        match x {
                            Piece::X => s.red(),
                            Piece::O => s.blue(),
                        }
                    })
                    .unwrap_or(" ".to_string().black())
            };

            print!("{piece} ");

            if j < 2 {
                print!("{}", "|".yellow());
            }
        }

        println!();

        if i < 2 {
            println!("{}", " - + - + - ".yellow());
        }
    }
}

#[allow(dead_code)]
pub fn print_stalemate(board: &Board) {
    for i in 0..3 {
        for j in 0..3 {
            print!(" ");

            let piece = board[(i, j)]
                .map(|x| x.to_string().white())
                .unwrap_or(" ".to_string().white());

            print!("{piece} ");

            if j < 2 {
                print!("{}", "|");
            }
        }

        if i == 1 {
            print!("{}", "   Tie!");
        }

        println!();

        if i < 2 {
            println!("{}", " - + - + - ");
        }
    }
}

#[allow(dead_code)]
pub fn print_victory(board: &Board, player: Piece) {
    let color = match player {
        Piece::X => |s: &str| s.red(),
        Piece::O => |s: &str| s.blue(),
    };

    for i in 0..3 {
        for j in 0..3 {
            print!(" ");

            let piece = board[(i, j)]
                .map(|x| color(&x.to_string()))
                .unwrap_or(" ".to_string().white());

            print!("{piece} ");

            if j < 2 {
                print!("{}", color("|"));
            }
        }

        if i == 1 {
            print!("{}", color(&format!("   {player} Wins!")));
        }

        println!();

        if i < 2 {
            println!("{}", color(" - + - + - "));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    #[ignore]
    fn print() {
        let board = Board::from_str("x x x o o o - - -").unwrap();
        print_board(&board, (1, 1));
    }

    #[test]
    #[ignore]
    fn stalemate() {
        let board = Board::from_str("x x x o o o - - -").unwrap();
        print_stalemate(&board);
    }

    #[test]
    #[ignore]
    fn wins() {
        let board = Board::from_str("x x x o o o - - -").unwrap();
        print_victory(&board, Piece::O);
    }
}
