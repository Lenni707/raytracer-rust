use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;
use crate::helper_func::{is_near_zero, random_in_unit_sphere, reflect};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool; // (attenuation = dämpfung)
}

// Matte Material => not reflecting

pub struct Lambertian {
    albedo: Color, // albedo = rückstrahlungsvermögen
}
 
impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { albedo: a }
    }
}
 
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        hitrecord: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hitrecord.normal + random_in_unit_sphere().normalize();

        if is_near_zero(scatter_direction) {
            scatter_direction = hitrecord.normal;
        }
 
        *attenuation = self.albedo;
        *scattered = Ray::new(hitrecord.point, scatter_direction);
        true
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal {
            albedo: a
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction().normalize(), hitrecord.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(hitrecord.point, reflected);
        true        
    }
}