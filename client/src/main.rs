mod client;
mod print;

use client::Client;
use colored::Colorize;
use core::game::{board::Board, piece::Piece, state::GameState};
use core::{request::Request, response::Response};
use print::{print_board, print_stalemate, print_victory};
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed as Or};
use std::sync::{Arc, Mutex};
use std::{env, thread};

fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let err_msg = "Args: <address:port>";
    let address = args.next().ok_or(err_msg)?;

    let (mut client, board, piece) = Client::new(&address)?;
    let board = Arc::new(Mutex::new(board));

    let x = Arc::new(AtomicUsize::new(1));
    let y = Arc::new(AtomicUsize::new(1));
    let prompt = match piece {
        Piece::X => "> ".red(),
        Piece::O => "> ".blue(),
    };

    let board_send = Arc::clone(&board);
    let mut client_send = client.clone();
    let x_send = Arc::clone(&x);
    let y_send = Arc::clone(&y);
    let stdout_send = io::stdout();
    let prompt_send = prompt.clone();

    thread::spawn(move || loop {
        match client_send.recv_response() {
            Ok(Response::Valid { board, state }) => {
                let mut board_lock = board_send.lock().unwrap();
                *board_lock = board;

                match state {
                    GameState::Playing => {
                        print_board(&board_lock, (x_send.load(Or), y_send.load(Or)));
                    }

                    GameState::Win(played_piece) => {
                        print_victory(&board_lock, played_piece);
                        *board_lock = Board::new();
                    }

                    GameState::Stalemate => {
                        print_stalemate(&board_lock);
                        *board_lock = Board::new();
                    }
                }
            }

            Ok(res) => println!("{res}"),

            _ => {}
        }

        print!("{prompt_send}");
        stdout_send.lock().flush().expect("Failed to flush stdout");
    });

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = String::new();

    loop {
        let (xx, yy) = (x.load(Or), y.load(Or));
        print_board(&board.lock().unwrap(), (xx, yy));

        print!("{prompt}");
        stdout.lock().flush().expect("Failed to flush stdout");
        stdin.read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_ref() {
            "w" => x.store(xx.checked_sub(1).unwrap_or(2), Or),
            "a" => y.store(yy.checked_sub(1).unwrap_or(2), Or),
            "s" => x.store((xx + 1) % 3, Or),
            "d" => y.store((yy + 1) % 3, Or),
            "q" => break,

            "e" => {
                let req = Request::Play { idx: (xx, yy) };
                client.send_request(req).ok();
            }

            _ => {}
        }

        input.clear();
    }

    Ok(())
}
