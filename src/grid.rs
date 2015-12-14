//use hex2d::{Coordinate};
extern crate num;
use noise::{open_simplex2,Seed};

use na::{Pnt3,Vec3,Vec2,Identity};
use nc::ray::{Ray,RayCast};
use nc::shape::{Cuboid};

use ::ui::Camera;
use ::input::mouse::Mouse;

pub const TILESIZE: f32 = 10.;
pub const MAPSIZE: usize = 50; // square

#[derive(Debug,Clone,Copy)]
pub struct Tile {
    //pub coord: Coordinate,
    pub kind: TileKind,
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TileKind {
    Grass,
    Water,
    Stone,
    Sand,
}

pub struct Grid {
    pub tiles: Vec<Tile>,
    pub size: usize,
}

impl Grid {
    pub fn new () -> Grid {
        let mut v = vec![Tile { kind: TileKind::Grass }; MAPSIZE*MAPSIZE];
        let g = Grid::gen(0,MAPSIZE,MAPSIZE);

        let mut i = 0;
        for _ in 0..MAPSIZE {
            for _ in 0..MAPSIZE {
                let t = Tile { kind: Grid::gen_tile(g[i]) };
                v[i] = t;
                i += 1;
            }
        }

        Grid { tiles: v,
               size: MAPSIZE }
    }

    pub fn regen(s: u32, w: usize, h: usize,
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

    pub fn gen(s: u32, w: usize, h: usize) -> Vec<f32> {
        let mut pixels: Vec<f32> = vec![0.;(w * h)];

        Grid::regen(s,w,h, pixels.as_mut_slice());

        pixels
    }

    pub fn gen_tile(n: f32) -> TileKind {
        if n > 0. {
            if n > 0.35 {
                TileKind::Stone
            }
            else { TileKind::Grass }
        }
        else {
            if n < -0.35 {
                TileKind::Water
            }
            else { TileKind::Sand }
        }
    }

    /// intersects ray, based on dimensions and cam position
    pub fn has_ray (&self,cam:&Camera, with_mouse: Option<(&Mouse,Vec2<f32>)>) -> bool {
        let size = MAPSIZE as f32;
        let cube = Cuboid::new(Vec3::new(size, 0., size));
        
        let r;
        
        if let Some(mouse) = with_mouse {
            r = cam.get_mouse_ray(mouse.0,mouse.1);
        }
        else {
            r = cam.get_ray();
        }

        if let Some(rr) = cube.toi_with_ray(&Identity::new(), &r, true) {
            true
        }
        else { false }
    }
}
