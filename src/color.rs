use glam::Vec3A;

pub fn write_color(data: &mut [u8], loc: usize, pixel_color: Vec3A) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.0 * r.clamp(0.0, 1.0)) as u8;
    let gbyte = (255.0 * g.clamp(0.0, 1.0)) as u8;
    let bbyte = (255.0 * b.clamp(0.0, 1.0)) as u8;

    data[loc] = rbyte;
    data[loc + 1] = gbyte;
    data[loc + 2] = bbyte;
}
