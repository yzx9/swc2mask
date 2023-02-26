use crate::vec::Vec3f;

pub trait SDF: Sync + Send {
    fn signed_distance(&self, p: Vec3f) -> f32;

    fn hit(&self, p: Vec3f) -> Hit {
        Hit {
            signed_distance: self.signed_distance(p),
            u: 0.0,
            v: 0.0,
        }
    }

    fn is_in(&self, p: Vec3f) -> bool {
        self.is_in_bounding_box(p) && self.signed_distance(p) < 0.0
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f);

    fn is_in_bounding_box(&self, p: Vec3f) -> bool {
        let (min, max) = self.bounding_box();
        p.x >= min.x && p.y >= min.y && p.z >= min.z && p.x <= max.x && p.y <= max.y && p.z <= max.z
    }
}

pub struct Hit {
    pub signed_distance: f32,
    pub u: f32,
    pub v: f32,
}
