#![feature(drain)]

extern crate thule;
use thule::{Interface,Events,GameState,Keyboard,Grid};

extern crate glium;
use glium::glutin::VirtualKeyCode;

extern crate nalgebra as na;
use na::Vec3;

fn main() {
    //let grid = Grid::gen_rand(50,50);
    //Grid::debug_prn(&grid,50);
    
    let mut iface = Interface::new(800,800);
    let game = GameState::new();
    
    'main: loop {
        let offset = move_cam(&iface.keyboard);
        iface.cam.pos = iface.cam.pos + offset;
        iface.update(&game);
        
        for e in iface.events.drain(..) {
            match e {
                Events::Quit => break 'main,
                Events::Zoom(z) => { // TODO: reset camera pos?
                    if z > 0. { iface.cam.zoom *= 1.1; }
                    else { iface.cam.zoom *= 0.9; }
                },
                _ => {},
            }
        }
    }
}


fn move_cam(kb: &Keyboard,) -> Vec3<f32> {
    let mut v = na::zero();
    let keys = kb.get_held_keys();
    
    if keys[VirtualKeyCode::Up as usize] {
        v = v + Vec3::new(10.,0.,10.)
    }
    if keys[VirtualKeyCode::Down as usize] {
        v = v + Vec3::new(-10.,0.,-10.)
    }
    if keys[VirtualKeyCode::Left as usize] {
        v = v + Vec3::new(10.,0.,-10.)
    }
    if keys[VirtualKeyCode::Right as usize] {
        v = v + Vec3::new(-10.,0.,10.)
    }

    v
}
