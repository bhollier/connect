use crate::*;

/// Defines the controller for a player
pub trait PlayerController<G: Grid> {
    fn pick_move(&mut self, grid: &G) -> usize;
}

/// Play a game using the two controllers until [Status::Win], [Status::Draw] or [NoSpaceError]
pub fn play<G: Grid>(
    grid: &mut G,
    p1: &mut dyn PlayerController<G>,
    p2: &mut dyn PlayerController<G>,
) -> Result<Status, NoSpaceError> {
    loop {
        let player: &mut dyn PlayerController<G> = match grid.current_player() {
            Player::P1 => p1,
            Player::P2 => p2,
        };
        let index = player.pick_move(&grid);
        match grid.drop(index) {
            Ok(Status::OnGoing) => {}
            r => return r,
        }
    }
}
