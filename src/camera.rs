use crate::ray::{Point3, Ray};
use crate::helper_func::{degrees_to_radians, random_in_unit_disk};
use glam::DVec3;

pub struct Camera {
    origin: Point3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vector_up: Point3, // use (0, 1, 0) for normal horizontal level camera // vector_up descirbes the rotation of the camera so if its tilted or not
        vertical_fov: f64, 
        aspect_ratio: f64,
        aperture: f64, // tuffes englishes word: deutsch = apertur => lichtdurchlässige öffnung in optischen systemen. in dem fall einer linse. beschreibt hier die größe dieser öffnung
        focus_dist: f64
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0); // h is a ratio to the distance so h is the tan of the degrees/2
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).normalize();
        let u = vector_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w; // kanns nicht erklären aber ich glaube ich habs gecheckt
 
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(); // ok basicly defocus works by having a lens and shooting rays from random positions on the lens, with a angle, so they eventually meet and center in one point in the scene. This point then becomes sharp and focussed
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

