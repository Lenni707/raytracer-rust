use glam::DVec3;

pub type Point3 = DVec3;
pub type Color = DVec3;

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: DVec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: DVec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
 
    pub fn origin(&self) -> Point3 {
        self.orig
    }
 
    pub fn direction(&self) -> DVec3 {
        self.dir
    }
 
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + ( self.dir * t )
    }
}
