use glam::Vec3A;
use rand::{thread_rng, Rng};

use crate::Facing;

// Newtype boilerplate.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3(pub Vec3A);

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3(Vec3A::new(x, y, z))
    }
}

impl From<Vec3A> for Point3 {
    fn from(item: Vec3A) -> Self {
        Self(item)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3(pub Vec3A);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(Vec3A::new(x, y, z))
    }
    pub fn random(min: f32, max: f32) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let r_vec = Vec3::random(-1.0, 1.0);
            let lensq = r_vec.0.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return Vec3(r_vec.0 / r_vec.0.length());
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let unit = Vec3::random_unit_vector();
        if unit.0.dot(normal.0) > 0.0 {
            unit
        } else {
            (-unit.0).into()
        }
    }

    pub fn set_facing(&mut self, dir: Facing) {
        if let Facing::Back = dir {
            self.0 = -self.0;
        }
    }

    pub fn near_zero(&self) -> bool {
        self.0.x.abs() < 1e8 && self.0.y.abs() < 1e8 && self.0.z.abs() < 1e8
    }
}

impl From<Vec3A> for Vec3 {
    fn from(item: Vec3A) -> Self {
        Self(item)
    }
}
