use rand::random;
use glam::DVec3;

// Konstanten

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

// funcs

pub fn degrees_to_radians(degrees: f64) -> f64 {
    // MATHE UNTTERICHT CARRY OMG
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::random()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_vec3() -> DVec3 {
    DVec3::new(random_double(), random_double(), random_double())
}

pub fn random_range_vec3(min: f64, max: f64) -> DVec3 {
    DVec3::new(
        random_double_range(min, max), random_double_range(min, max), random_double_range(min, max)
    )
}

pub fn random_in_unit_sphere() -> DVec3 { // einfach so lange random machen bis es passt
    loop {
        let p = random_range_vec3(-1.0, 1.0); // pick random point in unit cube
        if p.length_squared() >= 1.0 { // if point would be outside of a unit sphere, skip it
            continue;
        }
        return p;
    }
}