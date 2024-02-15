use colored::Colorize;
use core::game::{board::Board, piece::Piece};

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn print_board(board: &Board, idx: (usize, usize), msg: &str) {
    clear();
    println!("{msg}");

    for i in 0..3 {
        for j in 0..3 {
            print!(" ");
            let piece = if (i, j) == idx {
                board[(i, j)]
                    .map(|piece| piece.to_string())
                    .unwrap_or("_".to_string())
                    .white()
            } else {
                board[(i, j)]
                    .map(|piece| {
                        let color = match piece {
                            Piece::X => |s: &str| s.red(),
                            Piece::O => |s: &str| s.blue(),
                        };

                        color(&piece.to_string())
                    })
                    .unwrap_or(" ".to_string().yellow())
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

    println!();
}

fn print_str(board: &Board, msg: &str) -> String {
    let mut s = String::with_capacity(0x50);

    for i in 0..3 {
        for j in 0..3 {
            s.push(' ');
            let piece = board[(i, j)]
                .map(|x| x.to_string())
                .unwrap_or(" ".to_string());

            s.push_str(&piece);
            s.push(' ');

            if j < 2 {
                s.push('|');
            }
        }

        if i == 1 {
            s.push_str(&format!("   {msg}"));
        }

        s.push('\n');

        if i < 2 {
            s.push_str(" - + - + - ");
            s.push('\n');
        }
    }

    s
}

pub fn print_stalemate(board: &Board) {
    let s = print_str(board, "Tie!");
    clear();
    println!("\n{}", s.white());
}

pub fn print_victory(board: &Board, player: Piece) {
    let s = print_str(board, &format!("{player} Wins!"));

    let color = match player {
        Piece::X => |s: &str| s.red(),
        Piece::O => |s: &str| s.blue(),
    };

    clear();
    println!("\n{}", color(&s));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    #[ignore]
    fn print() {
        let board = Board::from_str("x x x o o o - - -").unwrap();
        print_board(&board, (1, 1), "msg");
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
