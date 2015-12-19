use ::Grid;
use na::{zero,Vec3};

pub struct GameState {
    pub map: Grid,
    pub player: Player,
}

impl GameState {
    pub fn new () -> GameState {
        let grid = Grid::new(None);
        GameState {
            map: grid,
            player: Player { pos: Vec3::new(1.,1.,1.) }
        }
    }
}

pub struct Player {
    pub pos: Vec3<f32>,
}
