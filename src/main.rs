#![feature(drain)]

extern crate thule;
use thule::{Interface,Events,GameState,Keyboard};

extern crate glium;
use glium::glutin::VirtualKeyCode;

extern crate nalgebra as na;
use na::Vec3;

fn main() {
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
    let keys = kb.get_held_keys();
    if keys[VirtualKeyCode::Up as usize] {
        Vec3::new(10.,0.,10.)
    }
    else if keys[VirtualKeyCode::Down as usize] {
        Vec3::new(-10.,0.,-10.)
    }
    else if keys[VirtualKeyCode::Left as usize] {
        Vec3::new(10.,0.,-10.)
    }
    else if keys[VirtualKeyCode::Right as usize] {
        Vec3::new(-10.,0.,10.)
    }
    else { na::zero() }
}
