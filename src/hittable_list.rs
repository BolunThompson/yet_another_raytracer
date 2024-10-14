// I suspect this isn't the most effcient way of implementing a list of Hittable objects,
// but it's the simplest

use core::range::RangeInclusive;
use std::rc::Rc;

use crate::{HitRecord, Hittable, Ray};

pub type HittableList = Vec<Rc<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: RangeInclusive<f32>) -> Option<HitRecord> {
        self.iter().fold(None, |closest, v| {
            closest
                .map(
                    // into to convert from old-style iterator to new
                    |closest_i| match v.hit(r, (ray_t.start..=closest_i.t).into()) {
                        Some(hr) if hr.t < closest_i.t => hr,
                        _ => closest_i,
                    },
                )
                .or_else(|| v.hit(r, ray_t))
        })
    }
}
