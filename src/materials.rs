use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool; // (attenuation = dämpfung)
}


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
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);
        true
    }
}