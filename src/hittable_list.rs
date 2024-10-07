// I suspect this isn't the most effcient way of implementing a list of Hittable objects,
// but it's the simplest

use std::rc::Rc;

use crate::{HitRecord, Hittable, Ray};

pub type HittableList = Vec<Rc<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        self.iter().fold(None, |closest, v| {
            closest
                .map(|closest_i| match v.hit(r, ray_tmin, closest_i.t) {
                    Some(hr) if hr.t < closest_i.t => hr,
                    _ => closest_i,
                })
                .or_else(|| v.hit(r, ray_tmin, ray_tmax))
        })
    }
}
