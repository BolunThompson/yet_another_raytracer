use crate::{Facing, HitRecord, Hittable, Point3, Ray, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        let radius = radius.max(0.0);
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        // oc is another intuitively meaningless but algebraically derived values
        // used in the calcuatoin for simplicity.
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
        if root <= ray_tmin || ray_tmax >= root {
            root = (h - sqrtd) / a;
            if root <= ray_tmin || ray_tmax >= root {
                return None;
            }
        }
        let point = r.at(root);
        let normal: Vec3 = ((r.at(root).0 - c) / self.radius).into();
        let facing = Facing::calculate(r, &normal);
        Some(HitRecord {
            t: root,
            point,
            normal,
            facing,
        })
    }
}
