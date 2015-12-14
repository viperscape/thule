use glium::glutin::Event as glutin_event;
use glium::{Display};
use na::{Vec2,};

use ::ui::{Target,Colors,Render,Camera};
use ::input::keyboard::Keyboard;
use ::input::mouse::Mouse;
use ::events::Events;
use ::GameState;

pub struct Interface {
    display: Display,
    pub render: Render,
    pub keyboard: Keyboard,
    mouse: Mouse,

    pub events: Vec<Events>,

    pub dt: f64,

    pub cam: Camera,
}

impl Interface {
    pub fn new (size_x: u32, size_y: u32) -> Interface {
        let mut display: Display = Target::new(size_x, size_y);
        let render = Render::new(&mut display);

        Interface {
            display: display,
            render: render,
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
            events: vec!(),
            dt: 0.0,
            cam: Camera::default(),
        }
    }

    pub fn update (&mut self,
                   game: &GameState) {
        let mut window_events = vec!();
        for e in self.display.poll_events() {
            window_events.push(e);
        }

        // handle a closed-window event
        for event in window_events.iter() {
            match *event {
                glutin_event::Closed => self.events.push(Events::Quit),
                //glutin_event::Resized(x,y) => {
                   //rebuild context? 
                //},
                _ => {},
            }
        }
        
        self.keyboard.update(&window_events,&mut self.events);
        if let Some(win_size) = self.get_win_size() {
            self.mouse.update(&window_events,
                              &mut self.events,
                              win_size);

            if game.map.has_ray(&self.cam,None) {
                println!("r!");
            }
        }

        self.dt = self.render.update(&mut self.display,
                                     Colors::grey_dark(),
                                     game,
                                     &self.cam);
    }

    pub fn get_display_mut (&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn get_win_size(&self) -> Option<Vec2<f32>> {
        if let Some(size) = Target::get_size(&self.display) {
            return Some(Vec2::new(size.0 as f32,size.1 as f32))
        }
        None
    }
}
