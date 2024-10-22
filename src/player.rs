pub mod player;
pub mod random;

pub use player::*;
pub use random::*;
use std::fmt;
use std::fmt::{Display, Formatter};

/// A player, either P1 or P2. Games always start with P1
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub const fn other(&self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Player::P1 => write!(f, "X"),
            Player::P2 => write!(f, "O"),
        }
    }
}
