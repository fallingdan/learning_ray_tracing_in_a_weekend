use std::io::Write;

use crate::vec3::Vec3;

type Color = Vec3;

pub fn write_color<W: Write>(output: &mut W, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r_byte: u32 = (255.999 * r) as u32;
    let g_byte: u32 = (255.999 * g) as u32;
    let b_byte: u32 = (255.999 * b) as u32;

    match write!(output, "{} {} {}", r_byte, g_byte, b_byte) {
        Ok(_) => {}
        Err(_) => eprintln!("Error writing for color {:?}", pixel_color),
    }
}
