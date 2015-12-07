use ::Grid;

pub struct GameState {
    pub map: Grid,
}

impl GameState {
    pub fn new () -> GameState {
        let grid = Grid::new();
        GameState { map: grid }
    }
}
