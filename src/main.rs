#![feature(drain)]

extern crate thule;
use thule::{Interface,Events,GameState,Grid,Keyboard};

extern crate glium;
use glium::glutin::VirtualKeyCode;

extern crate nalgebra as na;
use na::Vec3;

fn main() {
    let mut iface = Interface::new(800,800);
    let game = GameState::new();
    
    'main: loop {
        let offset = move_cam(&iface.keyboard);
        iface.cam_pos = iface.cam_pos + offset;
        iface.update(&game);
        
        for e in iface.events.drain(..) {
            match e {
                Events::Quit => break 'main,
                _ => {},
            }
        }
    }
}


fn move_cam(kb: &Keyboard,) -> Vec3<f32> {
    let keys = kb.get_held_keys();
    if keys[VirtualKeyCode::Up as usize] {
        Vec3::new(1.,0.,1.)
    }
    else if keys[VirtualKeyCode::Down as usize] {
        Vec3::new(-1.,0.,-1.)
    }
    else if keys[VirtualKeyCode::Left as usize] {
        Vec3::new(-1.,0.,-1.)
    }
    else if keys[VirtualKeyCode::Right as usize] {
        Vec3::new(1.,0.,-1.)
    }
    else { na::zero() }
}
