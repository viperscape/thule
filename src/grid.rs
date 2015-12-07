use hex2d::{Coordinate};

pub const TILESIZE: f32 = 10.;

#[derive(Debug)]
pub struct Tile {
    pub coord: Coordinate,
    kind: TileKind,
}

#[derive(Debug)]
pub enum TileKind {
    Grass,
    Water,
    Stone,
}

#[derive(Debug)]
pub struct Grid {
    pub tiles: Vec<Tile>,
}

impl Grid {
    pub fn new () -> Grid {
        let mut v = vec!();
        for x in 0..50 {
            for y in 0..50 {
                v.push(Tile {
                    coord: Coordinate::new(x, y),
                    kind: TileKind::Grass,
                });
            }
        }

        Grid { tiles: v, }
    }
}
