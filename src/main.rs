#![feature(drain)]

extern crate thule;
use thule::{Interface,Events,GameState};

fn main() {
    let mut iface = Interface::new(800,800);
    let game = GameState::new();
    
    'main: loop {
        iface.update(&game);
        
        for e in iface.events.drain(..) {
            match e {
                Events::Quit => break 'main,
                _ => {},
            }
        }
    }
}
