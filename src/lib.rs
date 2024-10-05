pub mod color;
pub mod ray;
pub mod render;

pub use color::write_color;
pub use ray::Ray;
pub use render::{hit_sphere, ray_color, RenderConfig};
