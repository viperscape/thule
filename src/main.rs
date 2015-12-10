#![feature(drain)]

extern crate thule;
use thule::{Interface,Events,GameState,Grid};

fn main() {
    let mut iface = Interface::new(800,800);
    let game = GameState::new();

    let grid = Grid::gen::<f32>();
    println!("{:?}",grid);
    
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
