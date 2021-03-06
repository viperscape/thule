use na::{self,
         Vec3,zero,
         Iso3,Rot3,
         Vec2,
         Pnt3,};
use nc::ray::{Ray,};
use ::input::mouse::Mouse;

pub struct Camera {
    offset: Vec3<f32>,
    pub pos: Vec3<f32>,
    iso: Iso3<f32>,
    pub zoom: f32,

    at: Vec3<f32>, // NOTE: this is for cam dir, ideally we'd turn iso into this, somehow
}

impl Camera {
    pub fn default () -> Camera {
        let offset = Vec3::new(40.,-40.,40.);
        let mut cam = Camera {
            offset: offset,
            pos: offset,
            iso: Iso3::new(zero(),zero()),
            zoom: 0.45,
            at: zero(),
        };

        cam.look_at(Vec3::new(0.,0.,0.));
        cam
    }

    pub fn new () -> Camera {
        Camera {
            offset: zero(),
            pos: Vec3::new(0.,0.,1.),
            iso: Iso3::new(zero(),zero()),
            zoom: 1.,
            at: zero(),
        }
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

    pub fn repos(&mut self, to:Vec3<f32>) {
        self.pos = to + self.offset;
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

