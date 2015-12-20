use ::{Grid,TileKind,MAPSIZE};
use na::{zero,Vec3,Vec2};
use clock_ticks::precise_time_s;

pub const MOVE_TIME: f64 = 0.095;

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
    time: f64,
}

impl Player {

    pub fn new() -> Player {
        Player {
            grid_pos: zero(),
            time: precise_time_s(),
        }
    }

    pub fn pos(&self,size: f32) -> Vec3<f32> {
        let off = (self.grid_pos.y & 1) as f32 * (size / 2.);
        Vec3::new((self.grid_pos.x as f32 * size + off) * 0.866,
                  0.,
                  self.grid_pos.y as f32 * size * 0.75)
    }

    pub fn shift(&mut self, offset: Vec2<i8>, grid: &Grid) {
        let time = precise_time_s();
        if time-self.time < MOVE_TIME { return }
        
        let mut pos = self.grid_pos;
        
        if offset.x < 0 {
            if pos.x > 0 {
                pos.x -= 1;
            }
        }
        else if offset.x > 0 {
            if pos.x < MAPSIZE-1 {
                pos.x += 1;
            }
        }
        
        if offset.y < 0 {
            if pos.y > 0 {
                pos.y -= 1;
            }
        }
        else if offset.y > 0 {
             if pos.y < MAPSIZE-1 {
                pos.y += 1;
             }
        }

        let tile = grid.tiles[pos.x][pos.y];
        if tile.kind != TileKind::Stone {
            self.grid_pos = pos;
            self.time = time;
        }
    }
}
