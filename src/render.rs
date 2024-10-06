// src/render.rs

use std::{error::Error, f32::consts::PI};

use crate::{ray::Ray, Color, Hittable, Point3, Vec3};
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
    pub camera_center: Point3,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub viewport_upper_left: Point3,
    pub pixel00_loc: Point3,
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
        let camera_center: Point3 = Vec3A::ZERO.into();
        let focal_length = 1.0;

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u: Vec3 = (viewport_u.0 / wt as f32).into();
        let pixel_delta_v: Vec3 = (viewport_v.0 / ht as f32).into();
        let viewport_upper_left = Point3(
            camera_center.0
                - Vec3A::new(0.0, 0.0, focal_length)
                - viewport_u.0 / 2.0
                - viewport_v.0,
        );
        let pixel00_loc: Point3 =
            (viewport_upper_left.0 + 0.5 * (pixel_delta_u.0 + pixel_delta_v.0)).into();

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

pub fn ray_color(world: &dyn Hittable, r: &Ray) -> Color {
    // FIX: No objects are ever getting hit
    if let Some(hr) = world.hit(r, 0.0, f32::INFINITY) {
        eprintln!("HIT");
        Color(0.5 * (hr.normal.0 + Vec3A::new(1.0, 1.0, 1.0)))
    } else {
        let unit_direction = r.direction.0.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Color(((1.0 - a) * Vec3A::ONE + a * Vec3A::new(0.5, 0.7, 1.0)).into())
    }
}

// TODO: where else to put this?
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
