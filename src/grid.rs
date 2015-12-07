use hex2d::{Coordinate};
use std::collections::HashMap;

pub const TILESIZE: f32 = 10.;
pub const MAPSIZE: usize = 10; // square

#[derive(Debug,Clone,Copy)]
pub struct Tile {
    //pub coord: Coordinate,
    pub kind: TileKind,
}

#[derive(Debug,Clone,Copy)]
pub enum TileKind {
    Grass,
    Water,
    Stone,
}

pub struct Grid {
    pub tiles: HashMap<(usize,usize),Tile>,
    pub size: usize,
}

impl Grid {
    pub fn new () -> Grid {
        let mut v = HashMap::new();
        
        for r in 0..MAPSIZE {
            for c in 0..MAPSIZE {
                let t = Tile { kind: TileKind::Grass };
                v.insert((r,c),t);
            }
        }

        Grid { tiles: v,
               size: MAPSIZE }
    }
}
