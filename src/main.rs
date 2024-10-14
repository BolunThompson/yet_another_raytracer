// TODO: Refactor to idiomatic rust.
// TODO: Should I be using pub(crate) instead of pub?

use std::{error::Error, rc::Rc};

use raytracer::{
    render::{CameraConfig, ImageConfig},
    Camera, Color, HittableList, Lambertian, Metal, Point3, Sphere,
};

fn main() -> Result<(), Box<dyn Error>> {
    let image_config = ImageConfig::new("out.png".to_owned(), 400, 225, 16.0 / 9.0);
    let camera_config = CameraConfig::image_default(&image_config);

    let m_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let m_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let m_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let m_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let camera = Camera::new(camera_config, image_config);

    let world: HittableList = vec![
        Rc::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            m_ground.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            m_center.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            m_left.clone(),
        )),
        Rc::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            m_right.clone(),
        )),
    ];
    camera.render_file(&world)
}
