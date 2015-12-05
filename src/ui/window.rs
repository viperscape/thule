use glium::glutin;
use glium::{Display};
use glium::DisplayBuild;

pub struct Target;
impl Target {
    pub fn new(width: u32, height: u32) -> Display {
        let display = glutin::WindowBuilder::new()
            .with_title("Project Mab".to_string())
            .with_dimensions(width, height)
            .with_vsync()
            .with_multisampling(8)
            .with_depth_buffer(24)
            .build_glium()
            .unwrap_or_else(|e| panic!("Error creating window: {}", e));
        display
    }

    pub fn get_size (display: &Display) -> Option<(u32,u32)> {
        if let Some(window) = display.get_window() {
            return window.get_inner_size_pixels()
        }
        None
    }
}
