#![allow(dead_code)]

use na::{
    ToHomogeneous,
    Iso3,
    Mat4,
    Ortho3,
    Vec2,Vec3,Vec4,
    one,
};

pub const FOV: f32 = 0.75;

pub struct Transforms {
    camera_to_screen: Mat4<f32>,
    world_to_camera : Mat4<f32>,
}

impl Transforms {
    /// straight forward ui placement with these camera-less transform
    pub fn default_ui (win_size: Vec2<f32>) -> Transforms {
        Transforms {
                camera_to_screen: ortho(win_size),
                world_to_camera : translation(Vec3::new(0.0,0.0,0.0)),
        }
    }
}

impl Transforms {
   /// to be used with a 2d-camera
    pub fn to_screen(&self, world_position: Vec2<f32>) -> Mat4<f32> {
        let world_position = Vec4::new(
            world_position.x,
            world_position.y,
            0.0,
            1.0,
        );

        let camera_position = self.world_to_camera * world_position;

        let camera_translation =
            Iso3::new(
                Vec3::new(camera_position.x, camera_position.y, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                )
            .to_homogeneous();

        self.camera_to_screen * camera_translation
    }
}

/// get new ortho transform matrix based on window size specified
pub fn ortho(win_size: Vec2<f32>) -> Mat4<f32> {
    let ortho = Ortho3::new(
        win_size.x, win_size.y,
        -1.0, 1.0
    );

    ortho.to_mat()
}

//straight translation, used for ui placement
fn translation(v: Vec3<f32>) -> Mat4<f32> {
    let translation = Iso3::new(
        Vec3::new(v.x, v.y, v.z),
        Vec3::new(0.0, 0.0, 0.0),
    );

    translation.to_homogeneous()
}
