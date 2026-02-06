mod color;
mod helper_func;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod camera;

use std::io;
use glam::DVec3;

use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    color::Color,
    ray::{Point3, Ray},
    sphere::Sphere,
    camera::Camera
};

// --- RAYTRACER ALLGEMEIN ---
// mehrere Rays die mit geometrischen Formen (in dem Fall nur Kugeln kollidieren)
// Rays sind dargestellt in form einer gerade => einer Gleichung
// heißt, für z.b die collision werden einfach immer Gleichungen gleichgestellt und aufgelöst

// --- HOW HITING SPHERES WORK ---
// checks if the specifc ray hit a sphere by solving a quadratic equation, to check for any t (any moment of time while the ray is moving), 
// where it could have intersected the sphere, bc

// ray.x^2 * ray.y^2 * ray.z^2 = radius^2 has to be satiesfied on a "collision"

// gleichung für linie & gleichung für geometrisches objekt => nach x (pos) auflösen für wo, wenn es kollidiert wo es kollidiert

// hoffe ich habs gecheckt ist viel zu kompliziert

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, helper_func::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(0.5, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // image measurements

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // world

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))); // objects: if it looks confusing für later lennart: Box::new macht einen smartpointer, der dann auf etwas mit dem trait Hittable zeigt und der rest ist von selbst verständlich
    world.add(Box::new(Sphere::new(Point3::new(0.0, -105.0, 1.0), 100.0)));

    // camera ( viewport)

    let cam = Camera::new(ASPECT_RATIO);

    // rendering

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rWorking, Line remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + helper_func::random_double()) / (IMAGE_WIDTH -1) as f64;
                let v = (j as f64 + helper_func::random_double()) / (IMAGE_HEIGHT -1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
