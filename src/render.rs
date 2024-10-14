use std::{
    error::Error,
    f32::consts::PI,
    io::{self, Write},
};

use crate::{ray::Ray, Color, Hittable, HittableList, Point3, Vec3};
use glam::Vec3A;
use image::RgbImage;
use itertools::iproduct;
use rand::Rng;

// TODO: figure out the bad practices I'm accidentally committing
// TODO: Move image to another file?

pub struct ImageConfig {
    pub out: String,
    pub wt: u32,
    pub ht: u32,
}

impl ImageConfig {
    pub fn new(out: String, wt: u32, ht: u32, aspect_ratio: f32) -> Self {
        let actual_ar = (wt as f32) / (ht as f32);
        if (actual_ar - aspect_ratio).abs() > 0.1 {
            panic!("aspect ratio isn't approximately equal to wt and ht");
        }
        ImageConfig { out, wt, ht }
    }

    pub fn create_buffer(&self) -> Vec<u8> {
        vec![0; (self.wt as usize) * (self.ht as usize) * 3]
    }

    pub fn save(&self, image_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        RgbImage::from_raw(self.wt, self.ht, image_data)
            .ok_or("Failed to create image buffer from raw data")?
            .save(&self.out)
            .map_err(|e| e.into())
    }
    pub const fn real_ar(&self) -> f32 {
        self.wt as f32 / self.ht as f32
    }
    /// returns iterator of all the (i, x, y pairs)
    pub fn inds<'a>(&'a self) -> impl Iterator<Item = (u32, u32)> + 'a {
        iproduct!(0..self.ht, 0..self.wt).map(|(y, x)| (x, y))
    }

    pub fn set_pixel(&self, image_data: &mut Vec<u8>, x: u32, y: u32, color: &Color) {
        let i = 3 * (y * self.wt + x) as usize;
        let pixel = &mut image_data.split_at_mut(i).1[..3];
        pixel.copy_from_slice(&color.rgb());
    }
}

pub struct CameraConfig {
    pub focal_length: f32,
    pub camera_center: Point3,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub samples_per_pix: u32,
    pub max_depth: i32,
}

impl CameraConfig {
    pub fn image_default(image: &ImageConfig) -> CameraConfig {
        CameraConfig {
            focal_length: 1.0,
            camera_center: Point3::new(0.0, 0.0, 0.0),
            viewport_height: 2.0,
            viewport_width: 2.0 * image.real_ar(),
            samples_per_pix: 100,
            max_depth: 50,
        }
    }
}

pub struct Camera {
    pub config: CameraConfig,
    pub image: ImageConfig,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    pixel_samples_scale: f32,
}

impl Camera {
    pub fn new(config: CameraConfig, image: ImageConfig) -> Self {
        let viewport_u = Vec3::new(config.viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -config.viewport_height, 0.0);
        let pixel_delta_u: Vec3 = (viewport_u.0 / image.wt as f32).into();
        let pixel_delta_v: Vec3 = (viewport_v.0 / image.ht as f32).into();
        let viewport_upper_left = Point3(
            config.camera_center.0
                - Vec3A::new(0.0, 0.0, config.focal_length)
                - viewport_u.0 / 2.0
                - viewport_v.0 / 2.0,
        );
        let pixel00_loc: Point3 =
            (viewport_upper_left.0 + 0.5 * (pixel_delta_u.0 + pixel_delta_v.0)).into();
        let pixel_samples_scale = 1.0 / config.samples_per_pix as f32;

        Self {
            config,
            image,

            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            pixel_samples_scale,
        }
    }

    pub fn render_file(&self, world: &HittableList) -> Result<(), Box<dyn Error>> {
        let data = self.render_data(world);
        self.image.save(data)
    }

    fn render_data(&self, world: &HittableList) -> Vec<u8> {
        let mut data = self.image.create_buffer();
        // TODO: Parallelize (should be easy)
        for (x, y) in self.image.inds() {
            if x == 0 {
                print!("\rScanlines remaining: {}\x1B[K", self.image.ht - y - 1);
            }
            io::stdout().flush().expect("can't flush");

            let pixel = self.sample_ray(x, y, &world);
            self.image.set_pixel(&mut data, x, y, &pixel);
        }
        println!();
        data
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let (xo, yo) = sample_square();
        let pixel_sample = self.pixel00_loc.0
            + (x as f32 + xo) * self.pixel_delta_u.0
            + (y as f32 + yo) * self.pixel_delta_v.0;
        let origin = self.config.camera_center;
        let direction = Vec3(pixel_sample - origin.0);
        Ray { origin, direction }
    }

    fn sample_ray(&self, x: u32, y: u32, world: &HittableList) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        for ray in (0..self.config.samples_per_pix).map(|_| self.get_ray(x, y)) {
            color.0 += ray_color(world, &ray, self.config.max_depth)
                .percent(self.pixel_samples_scale)
                .0;
        }
        color
    }
}

fn sample_square() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    (rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5)
}

fn ray_color(world: &dyn Hittable, r: &Ray, depth: i32) -> Color {
    // 0.001 to avoid "shadow acne" where due to fp errors the bounced off ray is
    // below the surface and intersects again
    if let Some(hr) = world.hit(r, (0.001..=f32::INFINITY).into()) {
        let black = Color::new(0.0, 0.0, 0.0);
        if depth == 0 {
            return black;
        }
        hr.mat.scatter(r, &hr).map_or(black, |(r, attn)| {
            ray_color(world, &r, depth - 1).attn(attn)
        })
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
