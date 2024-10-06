// src/main.rs

use raytracer::{ray_color, HittableList, Point3, Ray, RenderConfig, Sphere};
use std::{
    error::Error,
    io::{self, Write},
    rc::Rc,
};

// TODO: Refactor to idiomatic rust.
// TODO: Should I be using pub(crate) instead of pub?

fn main() -> Result<(), Box<dyn Error>> {
    let config = RenderConfig::new();
    let mut image_data = vec![0u8; config.imgsize];

    for (i, x, y) in config.inds() {
        if x == 0 {
            eprint!("\rScanlines remaining: {} ", config.ht - y - 1);
            io::stdout().flush()?;
        }

        let world: HittableList = vec![
            Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
            Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
        ];

        // TODO: Some of this should be moved to the render code
        let pixel_center = config.pixel00_loc.0
            + config.pixel_delta_u.0 * x as f32
            + config.pixel_delta_v.0 * y as f32;
        let ray_direction = pixel_center - config.camera_center.0;

        let r = Ray {
            origin: config.camera_center,
            direction: ray_direction.into(),
        };

        let pixel = &mut image_data.split_at_mut(i).1[..3];
        pixel.copy_from_slice(&ray_color(&world, &r).rgb());
    }
    eprintln!("\nDone! 🎉");

    config.save_image(image_data)
}
