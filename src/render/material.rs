use crate::vec::{self, Vec3f};
use std::sync::Arc;

pub trait Material: Sync + Send {
    fn hit(&self, u: f32, v: f32) -> Vec3f;
}

pub struct SolidColor {
    color: Vec3f,
}

impl SolidColor {
    pub fn new(color: Vec3f) -> Arc<dyn Material> {
        Arc::new(SolidColor { color })
    }
}

impl Material for SolidColor {
    fn hit(&self, _u: f32, _v: f32) -> Vec3f {
        self.color
    }
}

pub struct VAxisLinearGradient {
    c1: Vec3f,
    c2: Vec3f,
}

impl VAxisLinearGradient {
    pub fn new(c1: Vec3f, c2: Vec3f) -> Arc<dyn Material> {
        Arc::new(VAxisLinearGradient { c1, c2 })
    }
}

impl Material for VAxisLinearGradient {
    fn hit(&self, u: f32, _v: f32) -> Vec3f {
        let u = u.clamp(0.0, 1.0);
        vec::interpolate(self.c1, self.c2, u)
    }
}
