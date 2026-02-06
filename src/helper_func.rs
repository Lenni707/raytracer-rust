// Konstanten

pub use std::f64::INFINITY;
pub use std::f64::consts::PI;

// funcs

pub fn degrees_to_radians(degrees: f64) -> f64 {
    // MATHE UNTTERICHT CARRY OMG
    degrees * PI / 180.0
}
