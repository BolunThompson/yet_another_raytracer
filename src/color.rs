use glam::Vec3A;
// Right now `Color` is just for stronger type checking of Vec3As and helper methods.
// However, if I add additional members (ex: caching the rgb value),
// I would have to be careful about putting this in an array because that
// could lead to cache locality issues.
pub struct Color(pub Vec3A);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3A::new(r, g, b))
    }
    pub fn rgb(&self) -> [u8; 3] {
        let r = self.0.x;
        let g = self.0.y;
        let b = self.0.z;

        // translate the [0,1] component values to the byte range [0,255].
        let rbyte = (255.0 * r.clamp(0.0, 1.0)) as u8;
        let gbyte = (255.0 * g.clamp(0.0, 1.0)) as u8;
        let bbyte = (255.0 * b.clamp(0.0, 1.0)) as u8;

        [rbyte, gbyte, bbyte]
    }
}
