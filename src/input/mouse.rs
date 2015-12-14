#![allow(dead_code)]

use clock_ticks::precise_time_s;
use na::{Vec2,
         Pnt3,Vec3};

use glium::glutin::Event as glutin_events;
use glium::glutin::ElementState;
use glium::glutin::MouseButton;
use glium::glutin::MouseScrollDelta;
use glium::glutin::Event::{
    MouseMoved,
    MouseInput,
    MouseWheel,
};
use ::events::Events;
use ::ui::Camera;

const DRAGMIN_PX: i32 = 5i32;      // arbitrary 5px minimum
const DRAGMIN_TIME: f64 = 0.30f64; // 30ms time minimum

#[derive(Debug)]
pub struct Mouse {
    pos: (i32,i32),
    drag: (Option<(i32,i32)>,Option<(i32,i32)>),
    drag_start: f64,
    click: Option<(i32,i32)>,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: (0,0),
            drag: (None,None),
            drag_start: precise_time_s(),
            click: None,
        }
    }
    pub fn update ( &mut self,
                     window_events: &Vec<glutin_events>,
                     events: &mut Vec<Events>,
                     window_size: Vec2<f32>) {
        for event in window_events.iter() {
            match *event {
                MouseMoved(pos) => {
                    self.pos = pos;
                },
                MouseInput(ElementState::Pressed,MouseButton::Left) => {
                    self.drag.0 = Some(self.pos);
                    self.drag_start = precise_time_s();
                },
                MouseInput(ElementState::Released,MouseButton::Left) => {
                    if ((precise_time_s()-self.drag_start) > DRAGMIN_TIME) &
                        (((self.drag.0).unwrap().0 - self.pos.0).abs() >
                         DRAGMIN_PX) &
                        (((self.drag.0).unwrap().1 - self.pos.1).abs() >
                         DRAGMIN_PX)
                    {
                        self.drag.1 = Some(self.pos);
                        self.click = None;
                    }
                    else {
                        self.click = self.drag.0;
                        self.drag.0 = None;
                    }

                    self.handler(events,window_size);
                },
                MouseWheel(d) => {
                    match d {
                        MouseScrollDelta::LineDelta(_,y) => {
                            events.push(Events::Zoom(y as f32));
                        },
                        MouseScrollDelta::PixelDelta(_,y) => {
                            events.push(Events::Zoom(y as f32));
                        },
                    }
                },
                _ => { },
            }
        }
    }
    pub fn is_dragging (&self) -> bool {
        self.drag.0.is_some()
    }
    
    pub fn get_drag(&mut self) -> Option<((i32,i32),(i32,i32))> {
        if let Some(s) = self.drag.0 {
            if let Some(e) = self.drag.1 {
                let drag = Some((s,e));
                self.drag = (None,None);
                drag
            }
            else { None }
        }
        else { None }
    }

    pub fn get_click(&mut self) -> Option<(i32,i32)> {
        let click = self.click;
        self.click = None;
        click
    }

    fn handler(&mut self,
               events: &mut Vec<Events>,
               _win_size: Vec2<f32>,) {
            
        if let Some(click) = self.click {
            let click = Vec2::new(click.0 as f32, click.1 as f32);
            events.push(Events::MouseClick(click));
        }
        else if let Some(end) = self.drag.1 {
            let start = self.drag.0.unwrap();
            let start = Vec2::new(start.0 as f32,start.1 as f32);
            let end = Vec2::new(end.0 as f32,end.1 as f32);
            events.push(Events::MouseDrag((start,end)));
        }
    }

    /// determines if point is within other points
    fn within_bounds(p: f32, start: f32, end: f32) -> bool {
        let mut within = false;
        if start < end {
            if (p > start) &
                (p < end) { within = true; }
        }
        else {
            if (p < start) &
                (p > end) { within = true; }
        }

        within
    }

    /// converts mouse coordinate to world position
    pub fn convert_ui_coord(pos: Vec2<f32>, window_size: Vec2<f32>) -> Vec2<f32> {
        let x = pos.x - window_size.x / 2.0;
        let y = pos.y - window_size.y / 2.0;

        Vec2::new(x,-1.0*y)
    }

    /// returns base (start,dir) for building a ray
    pub fn get_ray (&self, win_size: Vec2<f32>, cam: &Camera, is_2d: bool) -> (Pnt3<f32>,Vec3<f32>) {
        let pos = Vec2::new(self.pos.0 as f32,self.pos.1 as f32);
        if is_2d {
            let coord = Mouse::convert_ui_coord(pos,win_size) * cam.zoom;
            let dir = Vec3::new(0.0,0.0,1.0); //NOTE: might need inv
            return (Pnt3::new(coord.x,coord.y,0.0),dir)
        }

        let pv = ::ui::transforms::Transforms::grid(win_size,cam).to_pv();

        let r = ::ui::transforms::unproject(pv, &pos, &win_size);
        (*cam.pos.as_pnt(),r.1)
    }
}
