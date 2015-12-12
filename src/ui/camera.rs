use na::{Vec3,zero,Iso3,Rot3};

pub struct Camera {
    pub pos: Vec3<f32>,
    iso: Iso3<f32>,
}

impl Camera {
    pub fn default () -> Camera {
        let mut cam = Camera {
            pos: Vec3::new(-20.,-20.,-20.),
            iso: Iso3::new(zero(),zero()),
        };

        cam.look_at(Vec3::new(-50.,0.1,-50.));
        cam
    }

    /// updates lookat iso transform
    pub fn look_at (&mut self, at: Vec3<f32>) {
        let at = at - self.pos;
        let rot = Rot3::look_at_z(&at,
                                  &Vec3::y());
        self.iso = Iso3 { translation: self.pos,
                          rotation: rot, };
    }

    pub fn update (&self,) -> Iso3<f32> {
        self.iso * Iso3::new(self.pos,zero())
    }
}

