use core::range::RangeInclusive;
use std::rc::Rc;

use crate::{material::Material, Facing, HitRecord, Hittable, Point3, Ray, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Rc<dyn Material>) -> Sphere {
        let radius = radius.max(0.0);
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord> {
        // oc is another intuitively meaningless but algebraically derived values
        // used in the calculation for simplicity.
        let oc = self.center.0 - r.origin.0;
        let a = r.direction.0.length_squared();
        let h = r.direction.0.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }
        let point = r.at(root);
        let mut normal: Vec3 = ((point.0 - self.center.0) / self.radius).into();
        let facing = Facing::calculate(r, &normal);
        normal.set_facing(facing);
        Some(HitRecord {
            t: root,
            point,
            normal,
            facing,
            mat: self.mat.clone(),
        })
    }
}
