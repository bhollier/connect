use connect::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::io::Error;
use text_io::read;

pub struct UserPlayer();

impl<G: Grid> PlayerController<G> for UserPlayer {
    fn pick_move(&mut self, grid: &G) -> usize {
        println!("{}", &grid);
        print!("Drop at ");
        let mut index = read!();
        while index >= G::WIDTH || !grid.has_space(index) {
            println!("Invalid move!");
            print!("Drop at ");
            index = read!();
        }
        index
    }
}

fn main() -> Result<(), Error> {
    let mut rng = SmallRng::from_entropy();
    let mut grid = BitboardGrid::new();
    match play(
        &mut grid,
        &mut UserPlayer(),
        &mut RandomPlayer::new(&mut rng),
    ) {
        Ok(Status::Draw) => {
            println!("{}", &grid);
            println!("Draw!");
            Ok(())
        }
        Ok(Status::Win(_)) => {
            println!("{}", &grid);
            println!("{} Won!", grid.current_player());
            Ok(())
        }
        Err(_) => panic!(),
        _ => Ok(()),
    }
}
