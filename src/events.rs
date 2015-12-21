use na::Vec2;

pub enum Events {
    Quit,

    MouseClick(Vec2<f32>),
    MouseDrag((Vec2<f32>,Vec2<f32>)),
    
    Zoom(f32),
}
