use glium::{Display,Surface};
use ::ui::{Color};

pub struct Render;

impl Render {
    pub fn update(window: &mut Display,
                  color: Color,) {
        let mut target = window.draw();
        target.clear_color_and_depth((color[0],
                                      color[1],
                                      color[2],
                                      1.0), 1.0);

        target.finish().unwrap();
    }
}
