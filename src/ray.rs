use glam::Vec3A;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    /// Computes the point along the ray at parameter `t`.
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}
