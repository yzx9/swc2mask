use super::object::Object;
use crate::vec::{self, Vec3f};
use std::sync::Arc;

pub trait Scene: Send + Sync {
    fn hit(&self, p: Vec3f) -> Vec3f;
    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)>;
    fn set_background(&mut self, background: Vec3f);
}

pub struct ObjectsScene {
    objects: Vec<Arc<dyn Object>>,
    background: Vec3f,
}

impl ObjectsScene {
    pub fn new() -> ObjectsScene {
        ObjectsScene {
            objects: Vec::new(),
            background: Vec3f::new(0.0, 0.0, 0.0),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Object>) {
        self.objects.push(object)
    }
}

impl Scene for ObjectsScene {
    fn set_background(&mut self, background: Vec3f) {
        self.background = background
    }

    fn hit(&self, p: Vec3f) -> Vec3f {
        self.objects
            .iter()
            .find_map(|obj| obj.hit(p))
            .unwrap_or(self.background)
    }

    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)> {
        match self.objects.len() {
            0 => None,
            _ => Some(self.objects.iter().fold(
                self.objects[0].bounding_box(),
                |(min, max), obj| {
                    let (obj_min, obj_max) = obj.bounding_box();
                    (vec::minimum(min, obj_min), vec::maximum(max, obj_max))
                },
            )),
        }
    }
}
