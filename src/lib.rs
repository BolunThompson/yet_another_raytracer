#![feature(let_chains, new_range_api)]

// TODO: Fix lib situation -- exporting everything feels wrong.

pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod render;
pub mod sphere;
pub mod types;

pub use color::Color;
pub use hittable::{Facing, HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use material::{Lambertian, Material, Metal};
pub use ray::Ray;
pub use render::Camera;
pub use types::{Point3, Vec3};

pub use sphere::Sphere;
