use glium::{Display,Surface};
use ::ui::{Color,Colors,Transforms};
use ::ui::{GlyphDrawer,TileDrawer,MeshDrawer,MapDrawer};
use ::ui::glyphs::Text;
use na::{Vec2,Vec3};
use clock_ticks::precise_time_s;

use ::ui::{Target,Camera};
use ::GameState;
use ::{TileKind,Tile,Grid};

const FRAME_SAMPLE: usize = 120;
const COLOR_SAND: Color = [1., 0.92156863, 0.8039216]; //FFEBCD

pub struct Render {
    pub text: GlyphDrawer,
    pub tile: Vec<TileDrawer>,
    pub person: MeshDrawer,

    pub map: MapDrawer,
    
    fps: Timing,
}

impl Render {
    pub fn new (display: &mut Display,) -> Render {
        
        let mut tiles = vec!();
        for _ in 0 .. ::GROUPSIZE {
            for _ in 0 .. ::GROUPSIZE {
                tiles.push(TileDrawer::new_from_path(
                    "assets/mesh/hex.obj",display));
            }
        }
        
        Render {
            text: GlyphDrawer::new_from_path(
                "assets/font/SourceCodePro-Regular-20",display),
            
            tile: tiles,

            person: MeshDrawer::new_from_path(
                "assets/mesh/person.obj",display),

            map: MapDrawer::new(display),

            fps: Timing::new(),
        }
    }
    pub fn update(&mut self,
                  display: &mut Display,
                  color: Color,
                  game: &GameState,
                  cam: &Camera,) -> f64 {
        let dt = precise_time_s()-self.fps.time;
        self.fps.time = precise_time_s();
        
        if let Some(win_size) = Target::get_size(display) {
            let win_size = Vec2::new(win_size.0 as f32,win_size.1 as f32);

            self.fps.log_frame_time();
            let frame_time_avg = self.fps.frame_time_avg;
            
            let mut target = display.draw();
            target.clear_color_and_depth((color[0],
                                          color[1],
                                          color[2],
                                          1.0), 1.0);
            
            let ui_view = Transforms::ui(win_size);
            let grid_view = Transforms::grid(win_size,&cam);

            let size = 100. * cam.zoom;
            let player_pos = game.player.pos(size);
            
            // iter 2d tiles
            let mut c = -1;
            for (g,tiles) in self.tile.iter_mut().enumerate() {
                
                for (i,tile) in tiles.inst.map().iter_mut().enumerate() {
                    c += 1;
                    let r = i/::GRIDSIZE;
                    if c > ::GRIDSIZE as isize - 1 { c = 0; }
                    
                    let game_tile = &game.map
                        .grids[g].1
                        .tiles[r][c as usize];

                    tile.color_fog = (color[0],
                                      color[1],
                                      color[2]);

                    let aposy = r + game.map.grids[g].0.y;
                    let aposx = c  as usize + game.map.grids[g].0.x;
                    
                    let tile_color = {
                        if game.player.grid_pos == Vec2::new(aposx,
                                                             aposy) {
                            Colors::yellow()
                        }
                        else {
                            Render::get_tile_color(&game_tile.0)
                        }
                    };
                    
                    tile.color = (tile_color[0],
                                  tile_color[1],
                                  tile_color[2]);

                    let mut pos = Grid::hex_pos(aposy,
                                            aposx,
                                            size);
                    pos.y = game_tile.1.terra * 100.;
                    tile.pos_tile = (pos.x,pos.y,pos.z);
                    tile.pos_player = (player_pos.x,
                                       player_pos.y,
                                       player_pos.z);
                }
            }

            

            
            for drawer in self.tile.iter_mut() {
                drawer.draw(Vec3::new(size,size,size),
                            grid_view.to_pv(),
                            &mut target);
            }
            
            self.person.draw(Vec3::new(size,size,size),
                             Colors::gold(),
                             grid_view.to_screen(player_pos),
                             &mut target,);

            // rebind minimap, incase we updated it
            let map = {
                if !game.map_view { (Vec2::new(100.,100.),
                                     Vec3::new(290.,-290.,0.)) }
                else { (Vec2::new(400.,400.),
                        Vec3::new(0.,0.,0.)) }
            };
            self.map.draw(map.0,
                          game.player.grid_pos,
                          ui_view.to_screen(map.1),
                          &game.minimap,
                          &mut target);


            

            self.text.push(Text {
                text: format!("fps:{:0.2?}",frame_time_avg),
                size: Vec2::new(1.,1.),
                color: Colors::black(),
                center: false,
                pos: Vec3::new(-390.,-390.,0.),
            });

            self.text.draw(ui_view.to_pv(),&mut target);

            target.finish().unwrap();

        }

        dt
    }

    pub fn get_tile_color(tile: &Tile) -> [f32;3] {
        if tile.kind == TileKind::Grass {
            Colors::green_spring() }
        else if tile.kind == TileKind::Stone {
            Colors::red_brick() }
        else if tile.kind == TileKind::Sand {
            COLOR_SAND }
        else if tile.kind == TileKind::Ice {
            Colors::blue_sky() }
        else if tile.kind == TileKind::Snow {
            Colors::white_ghost() }
        else { Colors::blue() }
        
    }
}


struct Timing {
    frame_time_avg: f64,
    _frame_times: [f64;FRAME_SAMPLE],
    frame_time_idx: usize,
    frame_start: f64,

    time: f64,
}

impl Timing {
    fn new () -> Timing {
        Timing {
            frame_time_avg: 0.0,
            _frame_times: [0.0;FRAME_SAMPLE],
            frame_time_idx: 0,
            frame_start: precise_time_s(),
            
            time: precise_time_s(),
        }
    }

    fn log_frame_time (&mut self) {
        self.frame_time_idx += 1;
        if self.frame_time_idx > 119 {
            self.frame_time_avg = self.frame_time_idx as f64 /
                (precise_time_s() - self.frame_start);

            // reset
            self.frame_start = precise_time_s();
            self.frame_time_idx = 0;
        }
    }
}
