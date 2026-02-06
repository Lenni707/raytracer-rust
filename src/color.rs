use glam::DVec3;
use std::io::Write;

pub type Color = DVec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // divide the color by the number of samples for the median (durschschnitt)
    // lowkey nicht gecheckt warum aber irgendwie müssen wir gamma correction machen ka irgendwie weird
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    let ir = (256. * r.clamp(0.0, 0.999)) as i32;
    let ig = (256. * g.clamp(0.0, 0.999)) as i32;
    let ib = (256. * b.clamp(0.0, 0.999)) as i32;

    writeln!(out, "{} {} {}", ir, ig, ib).expect("writing color");
}
