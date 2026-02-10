use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::color::Color;
use crate::helper_func::{is_near_zero, random_in_unit_sphere, reflect, refract, random_double};

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
    albedo: Color,
    blur: f64 // 1.0 => very matte, 0.0 => very shiny
}

impl Metal {
    pub fn new(color: Color, blur: f64) -> Metal {
        Metal {
            albedo: color,
            blur: if blur < 1.0 { blur } else { 1.0 }

        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction().normalize(), hitrecord.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(hitrecord.point, reflected + self.blur * random_in_unit_sphere()); // adds blur, which is random scatter, gets amplfied by self.blur
        scattered.direction().dot(hitrecord.normal) > 0.0
    }
}


pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric { index_of_refraction }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // schlick ist der goat. seine easy eqiation hilft um die spiegel faktor eines z.b glasseszu bestimmen, wenn ein licht von einem bestimmten winkel kommt
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let refraction_ratio = if hitrecord.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_dir = r_in.direction().normalize();
        let cos_theta = f64::min(-unit_dir.dot(hitrecord.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let refracted_dir = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() { // only sometimes refract, sometimes refract to get that nice frsel effect
            reflect(unit_dir, hitrecord.normal) 
        } else {
            refract(unit_dir, hitrecord.normal, refraction_ratio)
        };

        *attenuation = Color::new(1.0, 1.0, 1.0);
        *scattered = Ray::new(hitrecord.point, refracted_dir);
        true
    }    
}