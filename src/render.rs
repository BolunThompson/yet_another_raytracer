// src/render.rs

use std::error::Error;

use crate::ray::Ray;
use glam::Vec3A;
use image::RgbImage;
use itertools::iproduct;

pub struct RenderConfig {
    // image
    pub out: String,
    pub aspect_ratio: f32,
    pub wt: u32,
    pub ht: u32,
    pub imgsize: usize,

    // camera
    pub focal_length: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub camera_center: Vec3A,
    pub viewport_u: Vec3A,
    pub viewport_v: Vec3A,
    pub pixel_delta_u: Vec3A,
    pub pixel_delta_v: Vec3A,
    pub viewport_upper_left: Vec3A,
    pub pixel00_loc: Vec3A,
}

impl RenderConfig {
    pub fn new() -> Self {
        let out = "out.png".to_owned();
        let aspect_ratio = 16.0 / 9.0;
        let wt = 400;
        let ht = ((wt as f32 / aspect_ratio).floor() as u32).max(1);
        let imgsize = (wt * ht * 3) as usize;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (wt as f32 / ht as f32);
        let camera_center = Vec3A::ZERO;
        let focal_length = 1.0;

        let viewport_u = Vec3A::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3A::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / wt as f32;
        let pixel_delta_v = viewport_v / ht as f32;
        let viewport_upper_left =
            camera_center - Vec3A::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            out,
            aspect_ratio,
            wt,
            ht,
            imgsize,
            focal_length,
            viewport_height,
            viewport_width,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            viewport_upper_left,
            pixel00_loc,
        }
    }

    /// returns iterator of all the (i, x, y pairs)
    pub fn inds<'a>(&'a self) -> impl Iterator<Item = (usize, u32, u32)> + 'a {
        iproduct!(0..self.wt, 0..self.ht).map(|(x, y)| ((3 * (y * self.wt + x)) as usize, x, y))
    }

    pub fn save_image(&self, image_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        RgbImage::from_raw(self.wt, self.ht, image_data)
            .ok_or("Failed to create image buffer from raw data")?
            .save(&self.out)
            .map_err(|e| e.into())
    }
}

pub fn hit_sphere(center: Vec3A, radius: f32, r: &Ray) -> f32 {
    let oc = center - r.origin;
    let a = r.direction.length_squared();
    let h = r.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

pub fn ray_color(r: &Ray) -> Vec3A {
    let t = hit_sphere(Vec3A::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3A::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * (n + Vec3A::ONE);
    }
    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Vec3A::ONE + a * Vec3A::new(0.5, 0.7, 1.0)
}
