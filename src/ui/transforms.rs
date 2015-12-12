#![allow(dead_code)]
use ::ui::Camera;

use na::{
    ToHomogeneous,
    Iso3,
    Mat4,
    Ortho3,
    Vec2,Vec3,
    zero,
    Rot3,
    Inv,

    Persp3,
};

pub struct Transforms {
    proj: Mat4<f32>,
    view: Mat4<f32>,
}

impl Transforms {
    /// straight forward ui placement with these camera-less transform
    pub fn ui (win_size: Vec2<f32>) -> Transforms {
        Transforms {
            proj: ortho(win_size),
            view: translation(zero(),None),
        }
    }

    pub fn grid (win_size: Vec2<f32>,cam: &Camera) -> Transforms {
        let iso = cam.update();
        Transforms {
            proj: ortho(win_size),
            view: iso.to_homogeneous().inv().unwrap(),
        }
    }
}

impl Transforms {
    /// to be used with a 2d-camera, returns PVM matrix
    pub fn to_screen(&self, pos: Vec3<f32>) -> Mat4<f32> {
        let model = translation(Vec3::new(pos.x,pos.y,pos.z),
                                None);
        
        self.proj * self.view * model
    }
}

/// get new ortho transform matrix based on window size specified
pub fn ortho(win_size: Vec2<f32>) -> Mat4<f32> {
    let ortho = Ortho3::new(
        win_size.x, win_size.y,
        -1000., 1000.
        );

    ortho.to_mat()
}

//straight translation, used for ui placement
pub fn translation(v: Vec3<f32>, r: Option<Vec3<f32>>) -> Mat4<f32> {
    let translation = Iso3::new(
        Vec3::new(v.x, v.y, v.z),
        r.unwrap_or(zero()),
        );

    translation.to_homogeneous()
}

/// fov in hundreths (0.75, not 75.0)
pub fn persp(win_size: Vec2<f32>, fov: f32) -> Mat4<f32> {
    let persp = Persp3::new(
        win_size.x / win_size.y, fov,
        0.1, 1000.0
    );

    persp.to_mat()
}
