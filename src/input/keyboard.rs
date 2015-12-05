use glium::glutin::Event as glutin_events;
use glium::glutin::Event::{KeyboardInput};
use glium::glutin::VirtualKeyCode;
use glium::glutin::ElementState::{Pressed,Released};

use ::Events;

/// Keyboard Input Controller
pub struct Keyboard {
    held_keys: [bool;256], //keys currently being pressed
}

impl Keyboard {
    pub fn new () -> Keyboard {
        Keyboard { held_keys: [false;256] }
    }

    pub fn get_held_keys(&self) -> &[bool;256] {
        &self.held_keys
    }

    pub fn update(
        &mut self,
        window_events: &Vec<glutin_events>,
        events: &mut Vec<Events>)
    {
        for event in window_events.iter() {
            match *event {
                KeyboardInput(Pressed, _, Some(key)) => {
                    self.held_keys[key as usize] = true;
                },
                KeyboardInput(Released, _, Some(key)) => {
                    let nkey = key as usize;
                    self.held_keys[nkey] = false;

                    //special case for caps lock
                    if key == VirtualKeyCode::Capital {
                        self.held_keys[nkey] != self.held_keys[nkey];
                    }

                    //self.handle_keys_released(key,events);
                    events.push(Events::KeyPress(key));
                },
                _ => {},
            }
        }

        // always handle any possible held keys
        self.handle_keys_held();
    }

    fn handle_keys_held (&mut self,) {
        /*if self.held_keys[VirtualKeyCode::Tab as usize] {
            if self.held_keys[VirtualKeyCode::PageUp as usize] {
                camera.zoom(0.25);
            }
            else if self.held_keys[VirtualKeyCode::PageDown as usize] {
                camera.zoom(-0.25);
            }
        }*/
    }

    fn handle_keys_released(&mut self,
                            key: VirtualKeyCode,
                            events: &mut Vec<Events>) {
        match key {
            VirtualKeyCode::Escape => {
                events.push(Events::Quit);
            },
            _ => {},
        }
         /*   },
            VirtualKeyCode::F9 => {
                // debug key to send a random maneuver, while tab is pressed
                if self.held_keys[VirtualKeyCode::Tab as usize] {
                    let mut rng = thread_rng();
                    let a = sample(&mut rng, 1..359, 1);
                    
                    let direction_rad = (a[0] as f64).to_radians();

                    let game_time_s = {
                        if let Some(game_time_s) = frame.game_time_s {
                            game_time_s
                        }
                        else {return} };
                    let data = ManeuverData {
                        start_s   : game_time_s + 0.0,
                        duration_s: random::<u8>() as f64,
                        angle     : direction_rad,
                    };

                    events.push(InputEvent::ScheduleManeuver(data));
                }
            },
            _ => {},
        }*/
    }
}
