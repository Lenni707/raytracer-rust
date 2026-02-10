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

pub fn is_near_zero(vec: DVec3) -> bool { // cehcks if a value is near zero to avoid floating point issues and direct opposites happening bc of random number generation
    let limit = 1.0e-8;
    vec[0].abs() < limit && vec[1].abs() < limit && vec[2].abs() < limit
}

pub fn reflect(vector: DVec3, surface_normal: DVec3) -> DVec3 { // ok so i dont know how to explain but bassicly the vector reflects just two times n upwards from a surface so it gets switched around like just reversing a velocity but taking. in. regard a surface normal
    vector - 2.0 * vector.dot(surface_normal) * surface_normal
}

pub fn refract(vector: DVec3, surface_normal: DVec3, etai_over_eta: f64) -> DVec3 {
    let cos_theta = f64::min(-vector.dot(surface_normal), 1.0);  // theta = winkel
    let r_out_perpendicular = etai_over_eta * (vector + cos_theta * surface_normal);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perpendicular.length_squared())) * surface_normal;
    r_out_perpendicular + r_out_parallel
}