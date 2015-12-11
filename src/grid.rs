//use hex2d::{Coordinate};
extern crate num;
use noise::{open_simplex2,Seed};
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

    pub fn regen(s: u32, w: u32, h: u32,
                 b: &mut [f32]) {
        let seed = Seed::new(s);
        let mut i = 0;
        
        for y in 0..w {
            for x in 0..h {
                let value: f32 = open_simplex2(&seed,
                                               &[x as f32,
                                                 y as f32]);
                
                b[i] = value;
                i += 1;
            }
        }
    }

    pub fn gen(s: u32, w: u32, h: u32) -> Vec<f32> {
        let mut pixels: Vec<f32> = vec![0.;(w * h) as usize];

        Grid::regen(s,w,h, pixels.as_mut_slice());

        pixels
    }
}
