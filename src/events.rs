use na::Vec2;
use glium::glutin::VirtualKeyCode;

pub enum Events {
    Quit,

    MouseClick(Vec2<f32>),
    MouseDrag((Vec2<f32>,Vec2<f32>)),

    KeyPress(VirtualKeyCode),
    
    Zoom(f32),
}
