// src/main.rs

use raytracer::{ray_color, write_color, Ray, RenderConfig};
use std::{
    error::Error,
    io::{self, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = RenderConfig::new();
    let mut image_data = vec![0u8; config.imgsize];

    for (i, x, y) in config.inds() {
        if x == 0 {
            eprint!("\rScanlines remaining: {} ", config.ht - y - 1);
            io::stdout().flush()?;
        }

        let pixel_center =
            config.pixel00_loc + config.pixel_delta_u * x as f32 + config.pixel_delta_v * y as f32;
        let ray_direction = pixel_center - config.camera_center;

        let r = Ray {
            origin: config.camera_center,
            direction: ray_direction,
        };

        let pixel_color = ray_color(&r);
        // TODO: Is this idiomatic rust? Should I be using an iterator over
        // (uninitilized?) image_data instead?
        write_color(&mut image_data, i, pixel_color);
    }
    eprintln!("\nDone! 🎉");

    config.save_image(image_data)
}
