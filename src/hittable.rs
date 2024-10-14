use core::range::RangeInclusive;
use std::rc::Rc;

use crate::{material::Material, Point3, Ray, Vec3};

#[derive(Copy, PartialEq, Eq, Debug, Clone)]
pub enum Facing {
    Front,
    Back,
}

impl Facing {
    pub fn calculate(ray: &Ray, outward_normal: &Vec3) -> Facing {
        if ray.direction.0.dot(outward_normal.0) < 0.0 {
            Self::Front
        } else {
            Self::Back
        }
    }
}

pub struct HitRecord {
    // multiplier of the ray direction, indicating the hit position.
    pub t: f32,
    pub point: Point3,
    pub normal: Vec3,
    pub facing: Facing,

    pub mat: Rc<dyn Material>,
}

pub trait Hittable {
    #[allow(unused_variables)]
    fn hit(&self, r: &Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord> {
        None
    }
}
