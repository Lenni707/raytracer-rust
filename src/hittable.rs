use glam::DVec3;
// lets multiple parts of the programm own a value, in this case: the trait Material
use std::sync::Arc; // need to use arc, bc rc is not safe with multithreading

use crate::materials::Material;
use crate::ray::{Point3, Ray};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: DVec3,
    pub material: Option<Arc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default() // -> implements a default value for every type specified in a struct so for example for f64 -> 0.0
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: DVec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            // "vorne" oder "hinten" sozusagen
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    // impl send + sync for multithreading, to safely pass this trait to other threads
    // trait hittable, to implement for every hitable object -> mindblown 🤯
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
