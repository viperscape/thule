#![allow(dead_code)]

use na::{
    ToHomogeneous,
    Iso3,
    Mat4,
    Ortho3,
    Vec2,Vec3,Vec4,
    zero,
};

pub struct Transforms {
    proj: Mat4<f32>,
    view: Mat4<f32>,
}

impl Transforms {
    /// straight forward ui placement with these camera-less transform
    pub fn default_ui (win_size: Vec2<f32>) -> Transforms {
        Transforms {
            proj: ortho(win_size),
            view: translation(zero(),None),
        }
    }

    pub fn default_grid (win_size: Vec2<f32>) -> Transforms {
        Transforms {
            proj: ortho(win_size),
            view: translation(zero(),None),
        }
    }
}

impl Transforms {
    /// to be used with a 2d-camera, returns PVM matrix
    pub fn to_screen(&self, position: Vec2<f32>) -> Mat4<f32> {
        let position = Vec4::new(
            position.x,
            position.y,
            1.,
            1.,
            );

        let view_model = self.view * position;

        let view_model =
            Iso3::new(
                Vec3::new(view_model.x, view_model.y, view_model.z),
                zero(),
                )
            .to_homogeneous();

        self.proj * view_model
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
