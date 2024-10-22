use crate::{Grid, PlayerController};

/// Player which just picks a column (with space) at random
pub struct RandomPlayer<'rng, RNG: rand::Rng> {
    rng: &'rng mut RNG,
}

impl<'rng, RNG: rand::Rng> RandomPlayer<'rng, RNG> {
    pub fn new(rng: &'rng mut RNG) -> Self {
        RandomPlayer { rng }
    }
}

impl<'rng, G: Grid, RNG: rand::Rng> PlayerController<G> for RandomPlayer<'rng, RNG> {
    fn pick_move(&mut self, grid: &G) -> usize {
        let moves = grid.valid_moves();
        moves[self.rng.gen_range(0..moves.len())]
    }
}
