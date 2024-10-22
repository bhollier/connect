use crate::*;
use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;

/// A performant [Grid] using bitboards
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BitboardGrid {
    bitboards: [u64; 2],
    height: [u64; 7],
    counter: u8,
}

impl BitboardGrid {
    const TOP: u64 = 0b1000000100000010000001000000100000010000001000000;
    const DRAW_BITBOARD: u64 = 0b111111011111101111110111111011111101111110111111;

    pub fn new() -> Self {
        BitboardGrid {
            bitboards: [0, 0],
            height: [0, 7, 14, 21, 28, 35, 42],
            counter: 0,
        }
    }

    fn is_win(bitboard: u64) -> bool {
        if bitboard & (bitboard >> 6) & (bitboard >> 12) & (bitboard >> 18) != 0 {
            return true;
        }
        if bitboard & (bitboard >> 8) & (bitboard >> 16) & (bitboard >> 24) != 0 {
            return true;
        }
        if bitboard & (bitboard >> 7) & (bitboard >> 14) & (bitboard >> 21) != 0 {
            return true;
        }
        if bitboard & (bitboard >> 1) & (bitboard >> 2) & (bitboard >> 3) != 0 {
            return true;
        }
        false
    }
}

impl Display for BitboardGrid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Grid::fmt(self, f)
    }
}

impl Grid for BitboardGrid {
    const WIDTH: usize = 7;
    const HEIGHT: usize = 6;

    fn get(&self, x: usize, y: usize) -> &Token {
        if self.bitboards[0] >> ((x * 7) + y) & 1 == 1 {
            &Token::Filled(Player::P1)
        } else if self.bitboards[1] >> ((x * 7) + y) & 1 == 1 {
            &Token::Filled(Player::P2)
        } else {
            &Token::Empty
        }
    }

    fn current_player(&self) -> &Player {
        unsafe {
            match self.counter & 1 {
                0 => &Player::P1,
                1 => &Player::P2,
                _ => unreachable_unchecked(),
            }
        }
    }

    fn has_space(&self, x: usize) -> bool {
        Self::TOP & (1 << self.height[x]) == 0
    }

    fn drop(&mut self, x: usize) -> Result<Status, NoSpaceError> {
        if !self.has_space(x) {
            return Err(NoSpaceError());
        }
        let mov = 1 << self.height[x];
        self.height[x] += 1;
        self.bitboards[(self.counter & 1) as usize] ^= mov;
        if self.bitboards[0] | self.bitboards[1] == Self::DRAW_BITBOARD {
            return Ok(Status::Draw);
        }
        if Self::is_win(self.bitboards[(self.counter & 1) as usize]) {
            return Ok(Status::Win(*self.current_player()));
        }
        self.counter += 1;
        Ok(Status::OnGoing)
    }
}
