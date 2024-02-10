mod client;
mod print;

use client::Client;
use colored::Colorize;
use core::game::board::Board;
use core::game::piece::Piece;
use core::request::Request;
use core::response::Response;
use print::{clear, print_board};
use std::io::Write;
use std::{env, io};

fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    let _ = args.next().unwrap();

    let err_msg = "Args: <address:port>";
    let address = args.next().ok_or(err_msg)?;

    let mut client = Client::new(&address)?;
    let piece = client
        .connect()
        .map_err(|_| "Could not connect to server")?;

    let (mut x, mut y) = (1usize, 1usize);
    let mut board = Board::new();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = String::new();
    let input_str = match piece {
        Piece::X => "> ".red(),
        Piece::O => "> ".blue(),
    };

    loop {
        clear();
        print_board(&board, (x, y));

        print!("{input_str}");
        stdout.lock().flush().expect("Failed to flush stdout");
        stdin.read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_ref() {
            "w" => x = x.checked_sub(1).unwrap_or(2),
            "a" => y = y.checked_sub(1).unwrap_or(2),
            "s" => x = (x + 1) % 3,
            "d" => y = (y + 1) % 3,
            "q" => break,

            "e" => {
                let req = Request::Play { idx: (x, y) };
                client.send_request(req).ok();
            }

            _ => {}
        }

        // match client.recv_response() {
        //     Ok(Response::Invalid) => eprintln!("Invalid move"),
        //     Ok(Response::WaitingForPlayer) => eprintln!("Waiting for player"),
        //     Ok(Response::Connect { piece: _ }) => eprintln!("Connected"),
        //     Ok(Response::Valid { piece, idx }) => {
        //         board[idx] = Some(piece);
        //     }

        //     Err(e) => eprintln!("{}", e),
        // }

        input.clear();
    }

    Ok(())
}
