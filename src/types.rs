use glam::Vec3A;

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
    pub fn set_facing(&mut self, dir: Facing) {
        if let Facing::Back = dir {
            self.0 = -self.0;
        }
    }
}

impl From<Vec3A> for Vec3 {
    fn from(item: Vec3A) -> Self {
        Self(item)
    }
}
