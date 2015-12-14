use na::{self,
         Vec3,zero,
         Iso3,Rot3,
         Vec2,
         Pnt3,};

use nc::ray::{Ray,RayCast};
use nc::math::Point;

use ::input::mouse::Mouse;

pub struct Camera {
    pub pos: Vec3<f32>,
    iso: Iso3<f32>,
    pub zoom: f32,

    at: Vec3<f32>, // NOTE: this is for cam dir, ideally we'd turn iso into this, somehow
}

impl Camera {
    pub fn default () -> Camera {
        let mut cam = Camera {
            pos: Vec3::new(-40.,-40.,-40.),
            iso: Iso3::new(zero(),zero()),
            zoom: 1.0,
            at: zero(),
        };

        cam.look_at(Vec3::new(-100.,0.1,-100.));
        cam
    }

    /// updates lookat iso transform
    pub fn look_at (&mut self, at: Vec3<f32>) {
        self.at = at; // store for camera ray
        
        let at = at - self.pos;
        let rot = Rot3::look_at_z(&at,
                                  &Vec3::y());
        self.iso = Iso3 { translation: self.pos,
                          rotation: rot, };
    }

    /// get's direction pointing
    pub fn dir(&self) -> Vec3<f32> {
        na::normalize(&(self.at - self.pos))
    }

    pub fn update (&self,) -> Iso3<f32> {
        let mut iso = self.iso;
        iso.translation = self.pos;
        iso
    }

    pub fn get_mouse_ray (&self, mouse: &Mouse, win_size: Vec2<f32>) -> Ray<Pnt3<f32>> {
        let r = mouse.get_ray(win_size,
                              &self,false);
        Ray::new(r.0,r.1)
    }

    pub fn get_ray (&self,) -> Ray<Pnt3<f32>> {
        Ray::new(*self.pos.as_pnt(),
                 self.dir())
    }
}

