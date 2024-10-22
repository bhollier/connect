use crate::*;
use std::fmt;
use std::fmt::{Display, Formatter};

/// A generic, naively implemented [Grid],
/// with const generics defining the width, height and the number of tokens required for a win.
/// For standard Connect 4 the [BitboardGrid] performs much faster,
/// at the expense of being less flexible
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GenericGrid<const WIDTH: usize, const HEIGHT: usize, const WIN_TOKENS: usize> {
    current_player: Player,
    grid: [[Token; HEIGHT]; WIDTH],
}

impl<const WIDTH: usize, const HEIGHT: usize, const WIN_TOKENS: usize>
    GenericGrid<WIDTH, HEIGHT, WIN_TOKENS>
{
    pub fn new() -> Self {
        GenericGrid {
            current_player: Player::P1,
            grid: [[Token::default(); HEIGHT]; WIDTH],
        }
    }

    fn set(&mut self, x: usize, y: usize, t: Token) {
        self.grid[x][y] = t
    }

    fn check_tokens(&self, mut x: i64, mut y: i64, dx: i64, dy: i64, p: Player) -> bool {
        let mut count = 0;
        while count < WIN_TOKENS {
            if x < 0 || x >= WIDTH as i64 || y < 0 || y >= HEIGHT as i64 {
                return false;
            }
            if self.get(x as usize, y as usize) != &Token::Filled(p) {
                count = 0;
            } else {
                count += 1;
            }
            x += dx;
            y += dy;
        }
        true
    }

    fn is_win(&self) -> bool {
        // Horizontal
        for x in 0..WIDTH as i64 {
            if self.check_tokens(x, 0, 0, 1, self.current_player) {
                return true;
            }
        }
        // Vertical
        for y in 0..HEIGHT as i64 {
            if self.check_tokens(0, y, 1, 0, self.current_player) {
                return true;
            }
        }
        // Diagonals
        for y in 0..=HEIGHT - WIN_TOKENS {
            if self.check_tokens(0, y as i64, 1, 1, self.current_player)
                || self.check_tokens((WIDTH - 1) as i64, y as i64, -1, 1, self.current_player)
            {
                return true;
            }
        }
        for x in 0..=WIDTH - WIN_TOKENS {
            if self.check_tokens(x as i64, 0, 1, 1, self.current_player)
                || self.check_tokens(x as i64, (HEIGHT - 1) as i64, 1, -1, self.current_player)
            {
                return true;
            }
        }
        false
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const WIN_TOKENS: usize> Display
    for GenericGrid<WIDTH, HEIGHT, WIN_TOKENS>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Grid::fmt(self, f)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const WIN_TOKENS: usize> Grid
    for GenericGrid<WIDTH, HEIGHT, WIN_TOKENS>
{
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn get(&self, x: usize, y: usize) -> &Token {
        &self.grid[x][y]
    }

    fn current_player(&self) -> &Player {
        &self.current_player
    }

    fn drop(&mut self, x: usize) -> Result<Status, NoSpaceError> {
        if !self.has_space(x) {
            return Err(NoSpaceError());
        }
        let mut y = 0;
        while self.get(x, y) != &Token::Empty {
            y += 1;
        }
        self.set(x, y, Token::Filled(self.current_player));
        if self.is_win() {
            Ok(Status::Win(self.current_player))
        } else {
            for x in 0..WIDTH {
                if self.has_space(x) {
                    self.current_player = self.current_player.other();
                    return Ok(Status::OnGoing);
                }
            }
            Ok(Status::Draw)
        }
    }
}
