#![feature(let_chains)]

// TODO: Fix lib situation -- exporting everything feels wrong.

pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod ray;
pub mod render;
pub mod sphere;
pub mod types;

pub use color::Color;
pub use hittable::{Facing, HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use ray::Ray;
pub use render::{ray_color, RenderConfig};
pub use types::{Point3, Vec3};

pub use sphere::Sphere;
