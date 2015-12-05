use glium::glutin::Event as glutin_event;
use glium::{Display};
use na::Vec2;

use ::ui::{Target,Colors,Render};
use ::input::keyboard::Keyboard;
use ::input::mouse::Mouse;
use ::events::Events;

pub struct Interface {
    display: Display,
    pub keyboard: Keyboard,
    mouse: Mouse,

    pub events: Vec<Events>,

    pub dt: f64,
}

impl Interface {
    pub fn new (size_x: u32, size_y: u32) -> Interface {
        let display: Display = Target::new(size_x, size_y);

        let keyboard = Keyboard::new();
        
        Interface {
            display: display,
            keyboard: keyboard,
            mouse: Mouse::new(),
            events: vec!(),
            dt: 0.0,
        }
    }

    pub fn update (&mut self) {
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
        }

        Render::update(&mut self.display,
                       Colors::grey_dark());
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
