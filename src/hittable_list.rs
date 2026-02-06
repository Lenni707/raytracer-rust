use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>, // Box<T> is a smart pointer, that allowcates stuff on the HEAP (frau bening omg). Heißt also das der Vec<T> keinen fixxen Wert hat aber der SmartPointer fix ist oder so. Ich hab ehrlich nicht richtig ahnung
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        // self explainatory: add object to the vector of hitable objects
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_hitrecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            // check if anything is hit and set it as the closest, temporarely if maybe others are closer
            if object.hit(ray, t_min, closest, &mut temp_hitrecord) {
                hit_anything = true;
                closest = temp_hitrecord.t;
                *rec = temp_hitrecord.clone();
            }
        }

        hit_anything
    }
}
