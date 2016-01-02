use noise::{open_simplex2,Brownian2, Seed};

use na::{Vec3,Vec2,};
use ::ui::{Colorable};

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
    Snow,
    Ice,
}

#[derive(Debug,Clone)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
    //seed: BiomeSeed, // removeme?
}

impl Grid {
    pub fn new (seed: &BiomeSeed, start: Vec2<usize>) -> Grid {
        let mut v = vec![vec![Tile { kind: TileKind::Grass }; GRIDSIZE];GRIDSIZE];
        let g = Grid::gen(seed,start,
                          Vec2::new(GRIDSIZE,GRIDSIZE),);

        for (i,r) in g.iter().enumerate() {
            for (j,t) in r.iter().enumerate() {
                let tile = Biome::gen_tile(t);
                v[i][j] = Tile { kind: tile }
            }
        }

        Grid { tiles: v,
               }//seed: seed }
    }

    pub fn regen(s: &BiomeSeed, start: Vec2<usize>, size: Vec2<usize>,
                 b: &mut Vec<Vec<Biome>>) {

        // NOTE: precompute these in BiomeSeeds?

        
        for (i,r) in (start.y .. size.y+start.y).enumerate() {
            for (j,c) in (start.x .. size.x+start.x).enumerate() {
                let y = r as f32 * s.terra.1.y;
                let x = c as f32 * s.terra.1.x;

                let terra = Brownian2::new(open_simplex2, 4).
                    wavelength(16.0).
                    apply(&s.terra.0,&[x,y]);

                let y = r as f32 * s.humid.1.y;
                let x = c as f32 * s.humid.1.x;

                let humid = Brownian2::new(open_simplex2, 4).
                    wavelength(16.0).
                    apply(&s.humid.0,&[x,y]);

                let y = r as f32 * s.temp.1.y;
                let x = c as f32 * s.temp.1.x;

                let temp = Brownian2::new(open_simplex2, 4).
                    wavelength(16.0).
                    apply(&s.temp.0,&[x,y]);
                
                b[i][j] = Biome {
                    humid: humid,
                    temp: temp,
                    terra: terra,    
                };
            }
        }
    }

    // TODO: reuse vec in regen/gen for gridgroup
    pub fn gen(s: &BiomeSeed,
               start: Vec2<usize>,
               size: Vec2<usize>,) -> Vec<Vec<Biome>> {
        let mut pixels: Vec<Vec<Biome>> =
            vec![vec![ Biome::zero();size.y]; size.x];

        Grid::regen(s,start,size, &mut pixels);

        pixels
    }

    //pub fn gen_rand(w: usize, h: usize) -> Vec<Vec<f32>> {
    //    let s = random::<u32>();
    //    Grid::gen(s,w,h)
    //}

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

    // NOTE: this should be deprecated soon
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
    seed: BiomeSeed,
}

impl GridGroup {
    pub fn new(seed: Option<BiomeSeed>) -> GridGroup {
        let seed = seed.unwrap_or(BiomeSeed::default());
        let mut grids = vec!();

        for y in 0..GROUPSIZE {
            for x in 0..GROUPSIZE { 
                let coord = Vec2::new(x*GRIDSIZE,y*GRIDSIZE);
                let grid = Grid::new(&seed,coord);
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
            if pos.x > coord.x + GRIDSIZE * 2 {
                coord.x += GRIDSIZE * 3;
                
                let new_grid = Grid::new(&self.seed,*coord);
                *grid = new_grid;
            }
            else if (pos.x as isize) < coord.x as isize - GRIDSIZE as isize
            {
                let x = coord.x as isize - (GRIDSIZE * 3) as isize;
                if x >= 0 {
                    coord.x = x as usize;
                    let new_grid = Grid::new(&self.seed,*coord);
                    *grid = new_grid;
                }
            }

            if pos.y > coord.y + GRIDSIZE * 2 {
                coord.y += GRIDSIZE * 3;
                
                let new_grid = Grid::new(&self.seed,*coord);
                *grid = new_grid;
            }
            else if (pos.y as isize) < coord.y as isize - GRIDSIZE as isize
            {
                let y = coord.y as isize - (GRIDSIZE * 3) as isize;
                if y >= 0 {
                    coord.y = y as usize;
                    let new_grid = Grid::new(&self.seed,*coord);
                    *grid = new_grid;
                }
            }
        }
    }

    /// exports game map at larger size
    // TODO: create based on player position
    pub fn export (seed: Option<BiomeSeed>) -> ::image::DynamicImage {
        let mut seed = seed.unwrap_or(BiomeSeed::default());
        seed.temp.1 = seed.temp.1 * 10.;
        seed.humid.1 = seed.humid.1 * 10.;
        seed.terra.1 = seed.terra.1 * 10.;
        
        let wh = MAPSIZE/10;
        let m = Grid::gen(&seed,
                          Vec2::new(0,0),
                          Vec2::new(wh,wh),);
        let mut v = vec!();
        for n in m.iter() {
            for t in n.iter() {
                let tile = Tile { kind: Biome::gen_tile(t) };
                let b = ::ui::Render::get_tile_color(&tile).to_bytes(); {
                    v.push(b);
                }
            }
        }
        let mut img = ::image::ImageBuffer::new(wh as u32, wh as u32);

        let mut i = 0;
        for (_,_, pixel) in img.enumerate_pixels_mut() {
            *pixel = ::image::Rgb(v[i]);
            i += 1;
        }

        img = ::image::imageops::rotate180(&img);
        ::image::ImageRgb8(img)
    }
}

pub struct BiomeSeed {
    pub temp: (Seed,Vec2<f32>),
    pub humid: (Seed,Vec2<f32>),
    pub terra: (Seed,Vec2<f32>),
}

impl BiomeSeed {
    pub fn default () -> BiomeSeed {
        let terra_s = Seed::new(0);
        let terra_m = Vec2::new(0.05,0.05);

        let humid_s = Seed::new(1);
        let humid_m = Vec2::new(0.65,0.65);

        let temp_s = Seed::new(2);
        let temp_m = Vec2::new(0.01,0.025);
        
        BiomeSeed {
            temp: (temp_s,temp_m),
            humid: (humid_s,humid_m),
            terra: (terra_s,terra_m),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Biome {
    pub temp: f32,
    pub humid: f32,
    pub terra: f32,
}

impl Biome {
    pub fn zero() -> Biome {
        Biome {
            temp: 0.,
            humid: 0.,
            terra: 0.,
        }
    }
    pub fn gen_tile(&self) -> TileKind {
        if self.terra > 0. {
            if self.terra > 0.35 {
                if self.temp < -0.2 &&
                    self.humid > 0.45 {
                        TileKind::Snow
                    }
                else { TileKind::Stone }
            }
            else { TileKind::Grass }
        }
        else {
            if self.terra < -0.35 {
                if self.temp < -0.45 {
                    TileKind::Ice
                }
                else { TileKind::Water }
            }
            else { TileKind::Sand }
        }
    }
}
