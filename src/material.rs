use crate::{Color, HitRecord, Ray, Vec3};

// class lambertian : public material {
//   public:
//     lambertian(const color& albedo) : albedo(albedo) {}

//     bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
//     const override {
//         auto scatter_direction = rec.normal + random_unit_vector();
//         scattered = ray(rec.p, scatter_direction);
//         attenuation = albedo;
//         return true;
//     }

//   private:
//     color albedo;
// };

pub trait Material {
    #[allow(unused_variables)]
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    // TODO: The lambertian scatter is off
    fn scatter(&self, _r: &Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
        // adding the random unit vector leads to there being less scattering
        // when the cos of the ray and the surface normal is low
        let dir = Vec3(hr.normal.0 + Vec3::random_unit_vector().0);
        // if hr.normal ~= random_vector, then dir = zero leading to divs by zero
        let dir = if !dir.near_zero() { dir } else { hr.normal };
        let scattered = Ray::new(hr.point, dir);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz_r: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz_r: f32) -> Self {
        // fuzz_r = 0 means no fuzzing, 1.0 means fuzzing the whole ray (percentages)
        assert!(fuzz_r <= 1.0);
        Self { albedo, fuzz_r }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
        let reflect_v = reflect(&r.direction, &hr.normal);
        // reflect_v is normalized for the math to make sense with the unit vector fuzz sphere
        let reflect_v = Vec3(reflect_v.0.normalize() + Vec3::random_unit_vector().0 * self.fuzz_r);
        Some((Ray::new(hr.point, reflect_v), self.albedo))
    }
}

fn reflect(rv: &Vec3, n: &Vec3) -> Vec3 {
    Vec3(rv.0 - 2.0 * rv.0.dot(n.0) * n.0)
}
