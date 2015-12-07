use glium::{Display,Surface};
use ::ui::{Color,Colors,Atlas,Transforms,translation};
use ::ui::{GlyphDrawer,MeshDrawer,};
use na::{Vec2,Vec3};
use clock_ticks::precise_time_s;

use ::ui::Target;
use ::GameState;

const FRAME_SAMPLE: usize = 120;

pub struct Render {
    pub text: GlyphDrawer,
    pub tile: MeshDrawer,

    fps: Timing,
}

impl Render {
    pub fn new (display: &mut Display,) -> Render {
        Render {
            text: GlyphDrawer::new_from_path(
                "assets/font/UbuntuMono-20",display),
            
            tile: MeshDrawer::new_from_path(
                "assets/mesh/hex.obj",display),

            fps: Timing::new(),
        }
    }
    pub fn update(&mut self,
                  display: &mut Display,
                  color: Color,
                  game: &GameState,) -> f64 {
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

            let ui = Transforms::default_ui(win_size);
            
            for tile in game.map.tiles.iter() {
                let size = 10.;
                let pos = Vec2::new(tile.coord.x as f32 * size,
                                    tile.coord.y as f32 + size);
                self.tile.draw(Vec2::new(size,size),
                               Colors::green_spring(),
                               ui.to_screen(pos),
                               &mut target);
            }

            self.text.draw("thule",
                           Vec2::new(1.,1.),
                           Colors::grey_light(),
                           true,
                           ui.to_screen(Vec2::new(100.,100.)),
                           &mut target,);

            target.finish().unwrap();

        }

        dt
    }
}


struct Timing {
    frame_time_avg: f64,
    frame_times: [f64;FRAME_SAMPLE],
    frame_time_idx: usize,
    frame_start: f64,

    time: f64,
}

impl Timing {
    fn new () -> Timing {
        Timing {
            frame_time_avg: 0.0,
            frame_times: [0.0;FRAME_SAMPLE],
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
