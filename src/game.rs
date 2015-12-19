use ::Grid;
use na::{zero,Vec3,Vec2};

pub struct GameState {
    pub map: Grid,
    pub player: Player,
}

impl GameState {
    pub fn new () -> GameState {
        let grid = Grid::new(None);
        GameState {
            map: grid,
            player: Player::new(),
        }
    }
}

pub struct Player {
    pub grid_pos: Vec2<usize>,
}

impl Player {

    pub fn new() -> Player {
        Player {
            grid_pos: zero(),
        }
    }

    pub fn pos(&self, grid: &Grid) -> Vec3<f32> {
        Vec3::new(self.grid_pos.x as f32,
                  0.,
                  self.grid_pos.y as f32,)
    }

    pub fn shift(&mut self, offset: Vec2<i8>, grid: &Grid) {
        if offset.x < 0 {
            self.grid_pos.x -= 1;
        }
        else if offset.x > 0 { self.grid_pos.x += 1; }
        
        if offset.y < 0 {
            self.grid_pos.y -= 1;
        }
        else if offset.y > 0 { self.grid_pos.y += 1; }
    }
}
