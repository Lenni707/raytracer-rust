use std::io::Write;
use glam::DVec3;

pub type Color = DVec3;
 
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = (255.999 * pixel_color.x) as i32;
    let g = (255.999 * pixel_color.y) as i32;
    let b = (255.999 * pixel_color.z) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}