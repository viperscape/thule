#![feature(drain)]

extern crate thule;
use thule::{Interface,Events};

fn main() {
    let mut iface = Interface::new(800,800);
    
    'main: loop {
        iface.update();
        
        for e in iface.events.drain(..) {
            match e {
                Events::Quit => break 'main,
                _ => {},
            }
        }
        
    }
}
