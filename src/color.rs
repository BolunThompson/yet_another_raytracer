use glam::Vec3A;
// Right now `Color` is just for stronger type checking of Vec3As and helper methods.
// However, if I add additional members (ex: caching the rgb value),
// I would have to be careful about putting this in an array because that
// could lead to cache locality issues.
#[derive(Clone, Copy)]
pub struct Color(pub Vec3A);

fn to_gamma(linear: f32) -> f32 {
    if linear > 0.0 {
        linear.sqrt()
    } else {
        0.0
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color(Vec3A::new(r, g, b))
    }
    pub fn rgb(&self) -> [u8; 3] {
        let r = to_gamma(self.0.x);
        let g = to_gamma(self.0.y);
        let b = to_gamma(self.0.z);

        // translate the [0,1] component values to the byte range [0,255].
        let rbyte = (255.0 * r.clamp(0.0, 1.0)) as u8;
        let gbyte = (255.0 * g.clamp(0.0, 1.0)) as u8;
        let bbyte = (255.0 * b.clamp(0.0, 1.0)) as u8;

        [rbyte, gbyte, bbyte]
    }

    pub fn attn(&self, percent: Color) -> Color {
        Color(self.0 * percent.0)
    }

    pub fn percent(&self, percent: f32) -> Color {
        Color(self.0 * percent)
    }
}
