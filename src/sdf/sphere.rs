use super::SDF;
use crate::vec::Vec3f;

pub struct Sphere {
    center: Vec3f,
    radius: f32,
    bounding_box: (Vec3f, Vec3f),
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32) -> Self {
        Sphere {
            center,
            radius,
            bounding_box: (center - radius, center + radius),
        }
    }
}

impl SDF for Sphere {
    fn signed_distance(&self, p: Vec3f) -> f32 {
        sd_sphere(p, self.center, self.radius)
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }
}

fn sd_sphere(p: Vec3f, c: Vec3f, r: f32) -> f32 {
    (p - c).norm() - r
}
