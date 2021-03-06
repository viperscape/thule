use std::path::Path;
use std::fs::File;
use std::io::{Read,Write};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use ::{Grid,MAPSIZE,GridGroup};

use na::{zero,Vec3,Vec2};
use clock_ticks::precise_time_s;

pub const MOVE_TIME: f64 = 0.095;

pub struct GameState {
    pub player: Player,
    pub map: GridGroup,
    pub world: Grid,
    pub minimap: ::glium::Texture2d,
    pub map_view: bool,
}

impl GameState {
    pub fn new (display: &::glium::Display) -> GameState {
        let biome_seeds = ::grid::BiomeSeed::default();
        let m = {
            if let Some(mut f) = File::open(&Path::new("map.dat")).ok() {
                let mut b = vec!();
                let _ = f.read_to_end(&mut b);
                decode(&b[..]).unwrap()
            }
            else {
                let m = Grid::gen_map(Some(biome_seeds));
                if let Some(mut f) = File::create(&Path::new("map.dat")).ok() {
                    let b = encode(&m,SizeLimit::Infinite).unwrap();
                    let _ = f.write(&b);
                }
                m
            }
        };

        let img = {
            if let Some(img) = ::image::open(&Path::new("map.png")).ok() {
                img
            }
            else {
                let img = Grid::export(&m);
                
                let mut f = File::create(&Path::new("map.png")).unwrap();
                let _ = ::image::ImageRgb8(img.to_rgb()).
                    save(&mut f, ::image::PNG);
                
                img
            }
        };
        
        GameState {
            player: Player::new(),
            map: GridGroup::new(),
            world: m,
            minimap: ::glium::Texture2d::new(display,img).unwrap(),
            map_view: false,
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
        Grid::hex_pos(self.grid_pos.y,self.grid_pos.x,size)
    }

    /// this shifts the player, after checking bounds of map
    /// then will generate the next set of tiles in the grid
    pub fn shift(&mut self, offset: Vec2<isize>, _grids: &GridGroup)  {
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

        //let tile = &grid.tiles[pos.x][pos.y];
        //if tile.kind != TileKind::Stone {
            self.grid_pos = pos;
            self.time = time;
        //}
    }
}
