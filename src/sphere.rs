use crate::hittable::{HitRecord, Hittable};
use crate::ray::{Point3, Ray};
use std::rc::Rc;
use crate::materials::Material;

pub struct Sphere {
    sphere_center: Point3,
    sphere_radius: f64,
    sphere_material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            sphere_center: cen,
            sphere_radius: r,
            sphere_material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.sphere_center;

        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.sphere_radius * self.sphere_radius;

        let discriminant = half_b * half_b - a * c; // eigentlich müsste ich die wurzel ziehen aber ich weiß , wenn das positiv ist gibt es 2 Lösungen, wenn es 0 ist, eine Lösung und bei negativ keine Lösung

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = f64::sqrt(discriminant);

        let root1 = (-half_b - sqrt_d) / a; // wegen quadratic formula -> ist wie bei der pq Formel mit diesem einmal + und einmal -; +-
        let root2 = (-half_b + sqrt_d) / a;

        let root = if in_range(root1, t_min, t_max) {
            root1
        } else if in_range(root2, t_min, t_max) {
            root2
        } else {
            return false;
        };

        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.sphere_center) / self.sphere_radius;
        hit_record.set_face_normal(ray, outward_normal);
        hit_record.material = Some(self.sphere_material.clone());

        true // object was hit --> NEIN WAS OMG DESWEGEN TRUE 🤯 hab gerade emoji taste bei mac "gefunden" und wollte sie ausnutzen
    }
}

fn in_range(t: f64, min: f64, max: f64) -> bool {
    t > min && t < max
}
