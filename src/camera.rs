use crate::ray::{Point3, Ray};
use crate::helper_func::degrees_to_radians;
use glam::DVec3;

pub struct Camera {
    origin: Point3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vector_up: Point3, // use (0, 1, 0) for normal horizontal level camera // vector_up descirbes the rotation of the camera so if its tilted or not
        vertical_fov: f64, 
        aspect_ratio: f64
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0); // h is a ratio to the distance so h is the tan of the degrees/2
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w; // complex and i dont know how to explain but i think i get it

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

