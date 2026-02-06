use rand::Rng;

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