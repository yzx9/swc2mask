use super::Material;
use crate::{sdf::SDF, vec::Vec3f};
use std::sync::Arc;

pub trait Object: Sync + Send {
    fn hit(&self, p: Vec3f) -> Option<Vec3f>;
    fn bounding_box(&self) -> (Vec3f, Vec3f);
}

pub struct SDFObject {
    sdf: Box<dyn SDF>,
    material: Arc<dyn Material>,
}

impl SDFObject {
    pub fn new(sdf: Box<dyn SDF>, material: Arc<dyn Material>) -> Arc<SDFObject> {
        Arc::new(SDFObject { sdf, material })
    }
}

impl Object for SDFObject {
    fn hit(&self, p: Vec3f) -> Option<Vec3f> {
        let hit = self.sdf.hit(p);
        if hit.signed_distance <= 0.0 {
            Some(self.material.hit(hit.u, hit.v))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.sdf.bounding_box()
    }
}
