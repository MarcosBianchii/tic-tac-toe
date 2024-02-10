use core::game::board::Board;
use core::response::Response;
use core::{game::piece::Piece, request::Request};
use core::{read_str, write_str};
use std::{io, net::TcpStream};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    fn validate_address(address: &str) -> bool {
        let &[address, port] = address.split(":").collect::<Vec<_>>().as_slice() else {
            return false;
        };

        if port.is_empty() {
            return false;
        }

        let count = address
            .split('.')
            .map(|n| n.parse::<u8>())
            .flatten()
            .count();

        if count != 4 {
            return false;
        }

        port.parse::<u16>().is_ok()
    }

    fn connect(stream: &mut TcpStream) -> io::Result<(Piece, Board)> {
        let data = read_str(stream)?;
        let res: Response = serde_json::from_str(&data)?;

        match res {
            Response::Connect { piece, board } => Ok((piece, board)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to connect",
            )),
        }
    }

    pub fn new(address: &str) -> Result<(Self, Board, Piece), &'static str> {
        if !Self::validate_address(address) {
            return Err("Invalid IP address");
        }

        let Ok(mut stream) = TcpStream::connect(address) else {
            return Err("Could not connect to server");
        };

        let Ok((piece, board)) = Self::connect(&mut stream) else {
            return Err("Failed to connect to server");
        };

        Ok((Self { stream }, board, piece))
    }

    pub fn send_request(&mut self, req: Request) -> io::Result<()> {
        let json = serde_json::to_string(&req)?;
        write_str(&mut self.stream, &json)
    }

    pub fn recv_response(&mut self) -> io::Result<Response> {
        let data = read_str(&mut self.stream)?;
        let err = io::Error::new(io::ErrorKind::InvalidData, "Failed to deserialize response");
        serde_json::from_str(&data).map_err(|_| err)
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.try_clone().unwrap(),
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.send_request(Request::Disconnect).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_address() {
        assert!(Client::validate_address("127.0.0.1:8080"));
        assert!(Client::validate_address("255.255.255.255:1111"));
        assert!(Client::validate_address("0.0.0.0:65535"));

        assert!(!Client::validate_address("255.255.255.256:2222"));
        assert!(!Client::validate_address("0.0.0.0:65536"));
    }
}
