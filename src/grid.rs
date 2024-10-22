use crate::*;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

/// A token on a [Grid], or [Token::Empty]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Token {
    Empty,
    Filled(Player),
}

impl Default for Token {
    fn default() -> Self {
        Token::Empty
    }
}

/// The status of a game after a move
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Status {
    /// The game is still ongoing
    OnGoing,

    /// The game has ended in a draw
    Draw,

    /// The specified player won
    Win(Player),
}

/// Error if there is no space for a token to be dropped
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct NoSpaceError();

/// Defines a grid of tokens
pub trait Grid: Copy + Clone + Debug + Display {
    const WIDTH: usize;
    const HEIGHT: usize;

    /// Return the token at the given X and Y coordinate.
    /// X coordinates are left to right, and Y are from the bottom to the top
    fn get(&self, x: usize, y: usize) -> &Token;

    /// Return the player who is making the next move
    fn current_player(&self) -> &Player;

    /// Whether there is space to drop a token at the given column
    fn has_space(&self, x: usize) -> bool {
        self.get(x, Self::HEIGHT - 1) == &Token::Empty
    }

    /// Drops a token at the given column,
    /// returns a [Result] with either the [Status] of the game,
    /// or [NoSpaceError] if there's no space for the token in the column
    fn drop(&mut self, x: usize) -> Result<Status, NoSpaceError>;

    /// Generate a list of valid column indexes
    fn valid_moves(&self) -> Vec<usize> {
        let mut moves = Vec::with_capacity(Self::WIDTH);
        moves.extend((0..Self::WIDTH).filter(|x| self.has_space(*x)));
        moves
    }

    /// Create a basic text representation of the grid, intended to be used for [Display]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current player: {}", self.current_player())?;
        for x in 0..Self::WIDTH {
            write!(f, " ")?;
            write!(f, " {} ", x)?;
        }
        writeln!(f)?;
        write!(f, "┌")?;
        for x in 0..Self::WIDTH {
            write!(f, "───")?;
            if x != Self::WIDTH - 1 {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;
        for y in (0..Self::HEIGHT).rev() {
            write!(f, "│")?;
            for x in 0..Self::WIDTH {
                match self.get(x, y) {
                    &Token::Filled(p) => write!(f, " {} ", p)?,
                    &Token::Empty => write!(f, "   ")?,
                }
                if x != Self::WIDTH - 1 {
                    write!(f, "│")?;
                } else {
                    writeln!(f, "│")?;
                }
            }
            if y != 0 {
                write!(f, "├")?;
                for x in 0..Self::WIDTH {
                    write!(f, "───")?;
                    if x != Self::WIDTH - 1 {
                        write!(f, "┼")?;
                    }
                }
                writeln!(f, "┤")?;
            }
        }
        write!(f, "└")?;
        for x in 0..Self::WIDTH {
            write!(f, "───")?;
            if x != Self::WIDTH - 1 {
                write!(f, "┴")?;
            }
        }
        write!(f, "┘")?;
        Ok(())
    }
}
