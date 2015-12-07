use ::Grid;

pub struct GameState {
    map: Grid,
}

impl GameState {
    pub fn new () -> GameState {
        let grid = Grid::new();
        GameState { map: grid }
    }
}
