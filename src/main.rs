#![feature(drain)]
extern crate rand;

extern crate thule;
use thule::{Interface,Events,GameState,Keyboard,Grid};

extern crate glium;
use glium::glutin::VirtualKeyCode;

extern crate nalgebra as na;
use na::{Vec3,Vec2,zero};

fn main() {
    //let grid = Grid::gen_rand(50,50);
    //Grid::debug_prn(&grid,50);
    
    let mut iface = Interface::new(800,800);
    let mut game = GameState::new();
    
    'main: loop {
        check_keys(&iface.keyboard,&mut game);
        let offset = move_cam(&iface.keyboard);
        iface.cam.pos = iface.cam.pos + offset;

        let size = 100. * iface.cam.zoom;
        let offset = move_player(&iface.keyboard);
        game.player.shift(offset,&game.map);

        iface.cam.pos = game.player.pos(size) - 40.;
        
        
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

fn check_keys (kb: &Keyboard,gs: &mut GameState) {
    let keys = kb.get_released_keys();
    if keys[VirtualKeyCode::R as usize] {
        gs.map = Grid::new(Some(rand::random::<u32>()),zero());
    }
}

fn move_player(kb: &Keyboard,) -> Vec2<i8> {
    let mut v = na::zero();
    let keys = kb.get_held_keys();
    let up = Vec2::new(0,1);
    let down = Vec2::new(0,-1);
    let left = Vec2::new(1,0);
    let right = Vec2::new(-1,0);
    
    if keys[VirtualKeyCode::W as usize] {
        v = v + up + left
    }
    if keys[VirtualKeyCode::S as usize] {
        v = v + down + right
    }
    if keys[VirtualKeyCode::A as usize] {
        v = v + down + left
    }
    if keys[VirtualKeyCode::D as usize] {
        v = v + up + right
    }

    v
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
        v = v + Vec3::new(5.,0.,-5.)
    }
    if keys[VirtualKeyCode::Right as usize] {
        v = v + Vec3::new(-5.,0.,5.)
    }

    v
}
