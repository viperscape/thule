#![allow(dead_code)]
use ::ui::Camera;

use na::{
    self,
    ToHomogeneous,
    Iso3,
    Mat4,
    Ortho3,
    Vec2,Vec3,
    zero,
    Inv,

    Persp3,

    Pnt2,Pnt3,Pnt4,
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

    /// gets persp*view matrix
    pub fn to_pv(&self) -> Mat4<f32> {
        self.proj * self.view
    }
}

/// get new ortho transform matrix based on window size specified
pub fn ortho(win_size: Vec2<f32>) -> Mat4<f32> {
    let ortho = Ortho3::new(
        win_size.x, win_size.y,
        -2000., 2000.
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


/// Converts a point in 2d screen coordinates to a ray (a 3d position and a direction)
pub fn unproject(projview: Mat4<f32>,
                 coord: &Vec2<f32>,
                 win_size: &Vec2<f32>)
                 -> (Pnt3<f32>, Vec3<f32>) {
        let normalized_coord = Pnt2::new(
            2.0 * coord.x  / win_size.x - 1.0,
            2.0 * -coord.y / win_size.y + 1.0);

        let normalized_begin = Pnt4::new(normalized_coord.x, normalized_coord.y, -1.0, 1.0);
        let normalized_end   = Pnt4::new(normalized_coord.x, normalized_coord.y, 1.0, 1.0);

        let cam = projview.inv().unwrap();

        let h_unprojected_begin = cam * normalized_begin;
        let h_unprojected_end   = cam * normalized_end;

        let unprojected_begin: Pnt3<f32> = na::from_homogeneous(&h_unprojected_begin);
        let unprojected_end:   Pnt3<f32> = na::from_homogeneous(&h_unprojected_end);

        (unprojected_begin, na::normalize(&(unprojected_end - unprojected_begin)))
}
