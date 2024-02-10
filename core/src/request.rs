use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Request {
    Disconnect,
    Play { idx: (usize, usize) },
}
