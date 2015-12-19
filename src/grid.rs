//use hex2d::{Coordinate};
use rand::random;
use noise::{open_simplex2,Seed};

use na::{Pnt3,Vec3,Vec2,
         ToHomogeneous,Iso3, zero, Identity};
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
    pub tiles: Vec<Vec<Tile>>,
    pub size: usize,
}

impl Grid {
    pub fn new () -> Grid {
        let mut v = vec![vec![Tile { kind: TileKind::Grass }; MAPSIZE];MAPSIZE];
        let g = Grid::gen(11223344556677889900,MAPSIZE,MAPSIZE);

        for (i,n) in g.iter().enumerate() {
            for (j,m) in n.iter().enumerate() {
                let tile = Grid::gen_tile(m);
                v[i][j] = Tile { kind: tile };
            }
        }

        Grid { tiles: v,
               size: MAPSIZE }
    }

    pub fn regen(s: u32, w: usize, h: usize,
                 b: &mut Vec<Vec<f32>>) {
        let seed = Seed::new(s);
        let mut i = 0;
        
        for r in 0..h {
            for c in 0..w {
                let y = c as f32 * 0.05;
                let x = r as f32 * 0.05;
                let value: f32 = open_simplex2(&seed,
                                               &[x, y]);
                
                b[r][c] = value;
                i += 1;
            }
        }
    }

    pub fn gen(s: u32, w: usize, h: usize) -> Vec<Vec<f32>> {
        let mut pixels: Vec<Vec<f32>> = vec![vec![0.;h];w];

        Grid::regen(s,w,h, &mut pixels);

        pixels
    }

    pub fn gen_rand(w: usize, h: usize) -> Vec<Vec<f32>> {
        let s = random::<u32>();
        Grid::gen(s,w,h)
    }

    pub fn gen_tile(n: &f32) -> TileKind {
        if n > &0. {
            if n > &0.35 {
                TileKind::Stone
            }
            else { TileKind::Grass }
        }
        else {
            if n < &-0.35 {
                TileKind::Water
            }
            else { TileKind::Sand }
        }
    }

    /// intersects ray, based on dimensions and cam position
    pub fn has_ray (&self,cam:&Camera, with_mouse: Option<(&Mouse,Vec2<f32>)>) -> bool {
        let size = (self.size as f32 * 1. * cam.zoom) / 2.;
        let cube = Cuboid::new(Vec3::new(size, 1., size));
        
        let r;
        
        if let Some(mouse) = with_mouse {
            r = cam.get_mouse_ray(mouse.0,mouse.1);
        }
        else {
            r = cam.get_ray();
        }

        //let iso = Iso3::new(zero(),zero());
        let rr = cube.toi_with_ray(&Identity::new(), &r, true);
        if let Some(rr) = rr { println!("rr:{:?}",rr); }
        rr.is_some()
    }

    pub fn debug (v: &Vec<f32>) -> Vec<&str> {
        let mut t = vec!();
        for n in v {
            if n > &0. {
                if n > &0.5 {
                    t.push("^"); //peak
                }
                else {
                    t.push("|"); //grass
                }
            }
            else {
                if n > &-0.5 {
                    t.push("~"); //water
                }
                else {
                    t.push("*"); //surf
                }
            }
        }

        t
    }

    pub fn debug_prn(v: &Vec<Vec<f32>>, size: usize) {
        for n in v {
            let l = Grid::debug(n);
            let mut s = String::new();
            for c in l { s.push_str(c); }
            println!("{}",s);
        }
    }
}
