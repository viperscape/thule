#![feature(drain)]
extern crate rand;

extern crate thule;
use thule::{Interface,Events,GameState,Keyboard,
            Bindings,Default};

extern crate glium;
use glium::glutin::VirtualKeyCode;

extern crate nalgebra as na;
use na::{Vec3,Vec2,zero};

fn main() {
    let mut iface = Interface::new(800,800,Bindings::default());
    let mut game = GameState::new();
    
    'main: loop {
        check_keys(&mut game,&mut iface);
        let offset = move_cam(&iface.keyboard);
        iface.cam.pos = iface.cam.pos + offset;

        let size = 100. * iface.cam.zoom;
        let offset = move_player(&iface);
        game.player.shift(offset,&game.map);
        iface.cam.repos(game.player.pos(size));

        game.map.update(game.player.grid_pos);
        
        
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

fn check_keys (gs: &mut GameState,iface: &mut Interface) {
    let keys = iface.keyboard.get_released_keys();
    
   // if when("refresh",&iface,) {
   //     gs.map = Grid::new(Some(rand::random::<u32>()),zero());
   // }

    let size = 100. * iface.cam.zoom;
    if when("focus",&iface) {
        iface.cam.look_at(gs.player.pos(size));
    }

    if keys[VirtualKeyCode::F12 as usize] &
        keys[VirtualKeyCode::Escape as usize] {
            iface.events.push(Events::Quit);
        }
}


fn when_action(action: &str, iface: &Interface, held: bool) -> bool {
    let bindings = &iface.bindings;
    let keys = {
        if held { iface.keyboard.get_held_keys() }
        else { iface.keyboard.get_released_keys() }
    };
    if let Some(vkey) = bindings.get(action) {
        keys[*vkey as usize]
    }
    else { false }
}

fn when_held(action: &str, iface: &Interface,) -> bool {
    when_action(action,iface,true)
}
fn when(action: &str, iface: &Interface,) -> bool {
    when_action(action,iface,false)
}


// TODO: investigate how poor this hmap lookup is
fn move_player(iface: &Interface) -> Vec2<isize> {
    let mut v = na::zero();
    
    let up = Vec2::new(0,1);
    let down = Vec2::new(0,-1);
    let left = Vec2::new(1,0);
    let right = Vec2::new(-1,0);
    
    if when_held("move_up",&iface) {
        v = v + up + left
    }
    if when_held("move_down",&iface) {
        v = v + down + right
    }
    if when_held("move_left",&iface) {
        v = v + down + left
    }
    if when_held("move_right",&iface) {
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
