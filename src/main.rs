mod color;
mod ray;
 
use std::io;
 
use color::Color;
use ray::{ Ray, Point3 };
use glam::DVec3;

// --- HOW IT WORKS ---
// checks if the specifc ray hit a sphere by solving a quadratic equation, to check for any t (any moment of time while the ray is moving), where it could have intersected the sphere, bc 
// ray.x^2 * ray.y^2 * ray.z^2 = radius^2 has to be satiesfied on a "collision"

// hoffe ich habs gecheckt ist viel zu kompliziert

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool { 
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0 // atleast 1 result: 1 result => ray only "touches" the sphere (intersects once); 2 results => ray goes "through" the sphere (intersects twice)
}
 
fn ray_color(r: &Ray) -> Color { // calculates the ray "color" based on the y coordinate (based of the height of the scene)
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) { // ob eine kugel bei 0.0, 0.0 -1.0 mit dem radius 0.5 getroffen wurde
        return Color::new(1.0, 0.0, 0.0); // dann rot machen
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(0.5, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
 
fn main() {
    // image measurements
 
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
 
    // camera ( viewport)
 
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;
 
    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = DVec3::new(viewport_width, 0.0, 0.0);
    let vertical = DVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - DVec3::new(0.0, 0.0, focal_length);
 
    // rendering
 
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
 
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rWorking, Line remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin, 
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
 
    eprint!("\nDone.\n");
}