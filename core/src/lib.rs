pub mod game;
pub mod request;
pub mod response;

use std::io::{self, Read, Write};
use std::mem;

#[macro_export]
macro_rules! io_err {
    ($msg:expr) => {
        io::Error::new(io::ErrorKind::InvalidData, $msg)
    };
}

pub fn read_str<R: Read>(stream: &mut R) -> io::Result<String> {
    let mut size = [0; mem::size_of::<usize>()];
    stream.read_exact(&mut size)?;

    // Get amount of bytes to read from stream.
    let size = usize::from_be_bytes(size);
    let mut data = vec![0; size];

    stream.read_exact(&mut data)?;
    String::from_utf8(data).map_err(|_| io_err!("Failed to parse string"))
}

pub fn write_str<W: Write>(stream: &mut W, s: &str) -> io::Result<()> {
    stream.write_all(&s.len().to_be_bytes())?;
    stream.write_all(s.as_bytes())
}
