use glium::{Display,Surface};
use ::ui::{Color,Colors,Transforms};
use ::ui::{GlyphDrawer,TileDrawer,};
use na::{Vec2,Vec3};
use clock_ticks::precise_time_s;

use ::ui::{Target,Camera};
use ::GameState;
use ::{TileKind,Tile};

const FRAME_SAMPLE: usize = 120;

pub struct Render {
    pub text: GlyphDrawer,
    pub tile: TileDrawer,

    fps: Timing,
}

impl Render {
    pub fn new (display: &mut Display,) -> Render {
        Render {
            text: GlyphDrawer::new_from_path(
                "assets/font/UbuntuMono-20",display),
            
            tile: TileDrawer::new_from_path(
                "assets/mesh/hex.obj",display),

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
            let _frame_time_avg = self.fps.frame_time_avg;
            
            let mut target = display.draw();
            target.clear_color_and_depth((color[0],
                                          color[1],
                                          color[2],
                                          1.0), 1.0);

            let ui_view = Transforms::ui(win_size);
            let grid_view = Transforms::grid(win_size,&cam);

            let size = 40. * cam.zoom;
            /*for r in 0..game.map.size {    
                let off = (r & 1) as f32 * (size / 2.);
                for c in 0..game.map.size {
                    let tile = game.map.tiles.get(&(r,c)).unwrap();
                    let pos = Vec3::new((c as f32 * size) + off,
                                        0.,
                                        r as f32 * size * 0.866);
                    self.tile.inst[r+c].tile_pos = *pos.as_array();
                    
                }
             }*/

            let mut c = 0;
            for (i,tile) in self.tile.inst.map().iter_mut().enumerate() {
                let r = i/game.map.size;
                if c > game.map.size { c = 0; }

                if let Some(game_tile) = game.map.tiles.get(&(r,c)) {
                    let off = (r & 1) as f32 * (size / 2.);
                    let pos = Vec3::new((c as f32 * size) + off,
                                        0.,
                                        r as f32 * size * 0.866);
                    tile.pos_tile = (pos.x,pos.y,pos.z);
                    let color = Render::get_tile_color(&game_tile);
                    tile.color = (color[0],color[1],color[2]);
                }
                c += 1;
            }

            self.tile.draw(Vec3::new(size,size,size),
                           grid_view.to_pv(),
                           &mut target);
            
            self.text.draw("",
                           Vec2::new(1.,1.),
                           Colors::grey_light(),
                           true,
                           ui_view.to_screen(Vec3::new(100.,100.,0.)),
                           &mut target,);

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
            Colors::white_ghost() }
        else { Colors::blue_sky() }
        
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
