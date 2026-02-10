mod camera;
mod color;
mod helper_func;
mod hittable;
mod hittable_list;
mod materials;
mod ray;
mod sphere;

use std::io;

use std::sync::Arc; // need to use arc, bc rc is not safe with multithreading

use rayon::prelude::*; // tuff multithreading

use crate::{
    camera::Camera,
    color::Color,
    helper_func::*,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    materials::{Dielectric, Lambertian, Metal},
    ray::{Point3, Ray},
    sphere::Sphere,
};

// TODO:
// - eigenen code für eigenes bild schreibe
// - zum shader machen (auf der gpu laufen lassen)
// - vielleicht so ein interface oder so, damit mann einfacher objekte platzieren kann
// - mit interface dann auf webassembly compelieren zum ausprobieren

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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        // recursion depth so it cant go on forever or until it doesnt hit anything => STACK OVERFLOW OMG WAS HPI TUFF "DAS WÄRE DANN EIN STACK OVERFLOW 🤓"
        return Color::new(0.0, 0.0, 0.0); // if depth is reached return black
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, helper_func::INFINITY, &mut rec) {
        // bei einer kollisions scattern die rays in eine random richtung
        let mut atenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .material
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut atenuation, &mut scattered)
        {
            return atenuation * ray_color(&scattered, world, depth - 1); // creates new rays based on the atenuation (dämpfung) of the material
        }
        return Color::new(0.0, 0.0, 0.0);
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
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50; // limit für die rekursion von der erschaffung von neuen rays

    // world

    let mut world = final_scene();

    // camera ( viewport)

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let vertical_fov = 30.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vertical_fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // rendering

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rWorking, Line remaining: {} ", j);
        let pixel_colors: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                // for multithreading
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + helper_func::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + helper_func::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }
                pixel_color
            })
            .collect();
        for pixel_color in pixel_colors {
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
        eprint!("\nDone.\n");
    }
}

fn final_scene() -> HittableList {
    // just copied ts, will do one myself tomorrow
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = random_vec3() * random_vec3();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = random_range_vec3(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn basic_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        // ground
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Color::new(0.1, 0.8, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        // middle
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Color::new(0.4, 0.0, 0.8))),
    )));
    world.add(Box::new(Sphere::new(
        // left
        Point3::new(-1.1, 0.0, -1.0),
        0.5, // radius negative, for hollow sphere
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        // right
        Point3::new(1.1, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Color::new(0.1, 0.1, 0.1), 0.2)),
    )));
    world
}

fn tuff_scene() -> HittableList {
    let mut world = HittableList::new();
    world
}
