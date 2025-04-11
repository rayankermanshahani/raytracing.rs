use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(writer: &mut W, pixel_color: Color) -> std::io::Result<()> {
    // extract color components
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // translate the [0,1] component values to byte range [0,255]
    let rbyte = (255.99 * r) as i32;
    let gbyte = (255.99 * g) as i32;
    let bbyte = (255.99 * b) as i32;

    // write out the pixel color components with a newline to output stream
    writeln!(writer, "{} {} {}", rbyte, gbyte, bbyte)
}
