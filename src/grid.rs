//use hex2d::{Coordinate};
extern crate num;
use noise::{open_simplex2,Seed,Point2};
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

use self::num::{Float, NumCast};

fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num::traits::cast(val).unwrap()
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

    pub fn gen<T: Float + NumCast>() -> Vec<T> {
        let seed = Seed::new(0);
        let mut pixels: Vec<T> = Vec::with_capacity((100 * 100) as usize);

        for y in (0..100) {
            for x in (0..100) {
                let value: f32 = cast(open_simplex2(&seed, &[cast::<_,T>(x) - cast::<_,T>(100/2), cast::<_,T>(y) - cast::<_,T>(100/2)]));
                pixels.push(cast(clamp(value * 0.5 + 0.5, 0.0, 1.0) * 255.0));
            }
        }

        pixels
    }
}

fn clamp<F: Float>(val: F, min: F, max: F) -> F {
    match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    }
    }
