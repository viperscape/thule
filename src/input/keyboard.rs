use glium::glutin::Event as glutin_events;
use glium::glutin::Event::{KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::ElementState::{Pressed,Released};

/// Keyboard Input Controller
pub struct Keyboard {
    held_keys: [bool;256], //keys currently being pressed
    rel_keys: [bool;256], //keys currently being pressed
}

impl Keyboard {
    pub fn new () -> Keyboard {
        Keyboard { held_keys: [false;256],
                   rel_keys: [false;256] }
    }

    pub fn get_held_keys(&self) -> &[bool;256] {
        &self.held_keys
    }

    pub fn get_released_keys(&self) -> &[bool;256] {
        &self.rel_keys
    }

    pub fn update(
        &mut self,
        window_events: &Vec<glutin_events>,)
    {
        // reset released keys
        for key in self.rel_keys.iter_mut() {
            *key = false;
        }
        
        for event in window_events.iter() {
            match *event {
                KeyboardInput(Pressed, _, Some(key)) => {
                    self.held_keys[key as usize] = true;
                },
                KeyboardInput(Released, _, Some(key)) => {
                    let nkey = key as usize;
                    self.held_keys[nkey] = false;
                    self.rel_keys[nkey] = true;

                    //special case for caps lock
                    if key == VirtualKeyCode::Capital {
                        self.held_keys[nkey] != self.held_keys[nkey];
                    }
                },
                _ => {},
            }
        }
    }
}
