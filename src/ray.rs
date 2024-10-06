use crate::{Point3, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Computes the point along the ray at parameter `t`.
    pub fn at(&self, t: f32) -> Point3 {
        Point3(self.origin.0 + self.direction.0 * t)
    }
}
