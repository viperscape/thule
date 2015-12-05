use glium::{Display,Surface};
use ::ui::{Color,Colors,GlyphDrawer,Atlas,Transforms};
use na::Vec2;

use ::ui::Target;

pub struct Render {
    pub text: GlyphDrawer,
}

impl Render {
    pub fn new (display: &mut Display,) -> Render {
        let font = Atlas::new("assets/font/UbuntuMono-20").expect("Font atlas cannot load, missing fonts?");
        Render {
            text: GlyphDrawer::new(font,display),
        }
    }
    pub fn update(&mut self,
                  display: &mut Display,
                  color: Color,) {
        if let Some(win_size) = Target::get_size(display) {
            let win_size = Vec2::new(win_size.0 as f32,win_size.1 as f32);
            
            let mut target = display.draw();
            target.clear_color_and_depth((color[0],
                                          color[1],
                                          color[2],
                                          1.0), 1.0);

            let ui = Transforms::default_ui(win_size);
            self.text.draw("thule",
                           Vec2::new(1.,1.),
                           Colors::grey_light(),
                           true,
                           ui.to_screen(Vec2::new(100.,100.)),
                           &mut target,);

            target.finish().unwrap();

        }
    }
}
