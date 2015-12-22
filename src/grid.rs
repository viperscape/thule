//use hex2d::{Coordinate};
//use rand::random;
use noise::{open_simplex2,Seed};

use na::{Vec3,Vec2,};
//use nc::ray::{RayCast};
//use nc::shape::{Cuboid};

//use ::ui::Camera;
//use ::input::mouse::Mouse;
use std::collections::HashMap;

pub const TILESIZE: f32 = 100.;
pub const MAPSIZE: usize = 1000; // square
pub const GRIDSIZE: usize = 25;
pub const INSTSIZE: usize = GRIDSIZE * 3; // 3 grids, 75 square


#[derive(Debug,Clone)]
pub struct Tile {
    //pub coord: Coordinate,
    pub kind: TileKind,
}

#[derive(Debug,Clone,PartialEq)]
pub enum TileKind {
    Grass,
    Water,
    Stone,
    Sand,
}

#[derive(Clone)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
    //pub size: usize,
    seed: u32,
}

impl Grid {
    pub fn new (seed: Option<u32>, start: Vec2<usize>) -> Grid {
        let seed = seed.unwrap_or(0);
        let mut v = vec![vec![Tile { kind: TileKind::Grass }; GRIDSIZE];GRIDSIZE];
        let g = Grid::gen(seed,start,Vec2::new(GRIDSIZE,GRIDSIZE));

        for (i,n) in g.iter().enumerate() {
            for (j,m) in n.iter().enumerate() {
                let tile = Grid::gen_tile(m);
                v[i][j] = Tile { kind: tile }
            }
        }

        Grid { tiles: v,
               seed: seed }
    }

    // TODO: consider using octaves/brownian
    pub fn regen(s: u32, start: Vec2<usize>, size: Vec2<usize>,
                 b: &mut Vec<Vec<f32>>) {
        let seed = Seed::new(s);
        
        for r in start.y .. size.y {
            for c in start.x .. size.x {
                let y = r as f32 * 0.05;
                let x = c as f32 * 0.05;
                let value: f32 = open_simplex2(&seed,
                                               &[x, y]);
                
                b[r][c] = value;
            }
        }
    }

    pub fn gen(s: u32, start: Vec2<usize>, size: Vec2<usize>,) -> Vec<Vec<f32>> {
        let mut pixels: Vec<Vec<f32>> = vec![vec![0.;size.y];size.x];

        Grid::regen(s,start,size, &mut pixels);

        pixels
    }

    //pub fn gen_rand(w: usize, h: usize) -> Vec<Vec<f32>> {
    //    let s = random::<u32>();
    //    Grid::gen(s,w,h)
    //}

    // TODO: use multiple noise maps for biome
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

    /// returns the appropriate real-coordinates of a grid's coords
    pub fn hex_pos (r: usize, c: usize, size: f32) -> Vec3<f32> {
        let off = (r & 1) as f32 * (size / 2.);
        Vec3::new((c as f32 * size + off) * 0.866,
                  0.,
                  r as f32 * size * 0.75)
    }

  /*  /// intersects ray, based on dimensions and cam position
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
    }*/

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

    pub fn debug_prn(v: &Vec<Vec<f32>>,) {
        for n in v {
            let l = Grid::debug(n);
            let mut s = String::new();
            for c in l { s.push_str(c); }
            println!("{}",s);
        }
    }
}

pub struct GridGroup {
    pub grids: Vec<(Vec2<usize>,Grid)>,
}

impl GridGroup {
    pub fn new(seed: Option<u32>) -> GridGroup {
        let mut grids = vec!();

        for i in 0..3 {
            for j in 0..3 {
                let coord = Vec2::new(j*GRIDSIZE,i*GRIDSIZE);
                let grid = Grid::new(seed,coord);
                grids.push((coord,grid));
            }
        }

        GridGroup { grids: grids }
    }

    /// updates grids based on player position
    /// this determines where the player is in terms of grid &
    /// position in list of gridgroup
    /// then finds which grid side to build and reposition for the
    /// grid instances
    pub fn update(&mut self,pos:Vec2<usize>) {
        for &mut (coord,ref grid) in self.grids.iter_mut() {
            if pos.x > (coord.x * GRIDSIZE) + GRIDSIZE / 2 {
               // println!("oob x!");
            }

            if pos.y > (coord.y * GRIDSIZE) + GRIDSIZE / 2 {
               // println!("oob y!");
            }
        }
    }
}
