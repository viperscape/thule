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
pub const GROUPSIZE: usize = 3;


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

#[derive(Debug,Clone)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
    //pub size: usize,
    seed: u32,
}

impl Grid {
    pub fn new (seed: u32, start: Vec2<usize>) -> Grid {
        let mut v = vec![vec![Tile { kind: TileKind::Grass }; GRIDSIZE];GRIDSIZE];
        let g = Grid::gen(seed,start,Vec2::new(GRIDSIZE,GRIDSIZE));

        for (i,r) in g.iter().enumerate() {
            for (j,t) in r.iter().enumerate() {
                let tile = Grid::gen_tile(t);
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
        
        for (i,r) in (start.y .. size.y+start.y).enumerate() {
            for (j,c) in (start.x .. size.x+start.x).enumerate() {
                let y = r as f32 * 0.05;
                let x = c as f32 * 0.05;
                let value: f32 = open_simplex2(&seed,
                                               &[x, y]);
                
                b[i][j] = value;
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

#[derive(Debug)]
pub struct GridGroup {
    pub grids: Vec<(Vec2<usize>,Grid)>,
    seed: u32,
}

impl GridGroup {
    pub fn new(seed: Option<u32>) -> GridGroup {
        let seed = seed.unwrap_or(0);
        let mut grids = vec!();

        for y in 0..GROUPSIZE {
            for x in 0..GROUPSIZE { 
                let coord = Vec2::new(x*GRIDSIZE,y*GRIDSIZE);
                let grid = Grid::new(seed,coord);
                grids.push((coord,grid));
            }
        }
        
        GridGroup { grids: grids,
                    seed: seed }
    }

    /// updates grids based on player position
    /// this determines where the player is in terms of grid &
    /// position in list of gridgroup
    /// then finds which grid side to build and reposition for the
    /// grid instances
    pub fn update(&mut self,pos:Vec2<usize>) {
        for &mut (ref mut coord,ref mut grid) in self.grids.iter_mut() {
            if pos.x > coord.x + GRIDSIZE + (GRIDSIZE/2) {
                coord.x += GRIDSIZE * 3;
                
                let new_grid = Grid::new(self.seed,*coord);
                *grid = new_grid;
            }
            else if (pos.x as isize) < (coord.x as isize
                                      - GRIDSIZE as isize
                                      - (GRIDSIZE/2) as isize)
            {
                let x = coord.x;
                if x as isize - (GRIDSIZE * 3) as isize > GRIDSIZE as isize {
                    coord.x -= GRIDSIZE * 3;
                }
                
                let new_grid = Grid::new(self.seed,*coord);
                *grid = new_grid;
            }

            if pos.y > coord.y + GRIDSIZE + (GRIDSIZE/2) {
                coord.y += GRIDSIZE * 3;
                
                let new_grid = Grid::new(self.seed,*coord);
                *grid = new_grid;
            }
            else if (pos.y as isize) < (coord.y as isize
                                        - GRIDSIZE as isize
                                        - (GRIDSIZE/2) as isize)
            {
                let y = coord.y;
                if y as isize - (GRIDSIZE * 3) as isize > GRIDSIZE as isize {
                    coord.y -= GRIDSIZE * 3;
                }
                
                let new_grid = Grid::new(self.seed,*coord);
                *grid = new_grid;
            }
        }
    }
}
