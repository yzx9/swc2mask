use super::{
    accelerator::{Accelerator, BVH},
    object::Object,
};
use crate::vec::{self, Vec3f};
use std::sync::Arc;

pub trait Scene: Send + Sync {
    fn hit(&self, p: Vec3f) -> Vec3f;
    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)>;
    fn set_background(&mut self, background: Vec3f);
}

pub struct ObjectsScene {
    objects: Option<Vec<Arc<dyn Object>>>,
    background: Vec3f,
    acceletor: Option<Box<dyn Accelerator>>,
}

impl ObjectsScene {
    pub fn new() -> ObjectsScene {
        ObjectsScene {
            objects: Some(Vec::new()),
            background: Vec3f::new(0.0, 0.0, 0.0),
            acceletor: None,
        }
    }

    pub fn add(&mut self, object: Arc<dyn Object>) {
        match &mut self.objects {
            Some(objs) => objs.push(object),
            None => panic!("scene is not editable"),
        }
    }

    pub fn build_bvh(&mut self) {
        match self.objects.take() {
            Some(objs) => self.acceletor = Some(BVH::new(objs)),
            None => panic!("scene is not editable"),
        }
    }
}

impl Scene for ObjectsScene {
    fn set_background(&mut self, background: Vec3f) {
        self.background = background
    }

    fn hit(&self, p: Vec3f) -> Vec3f {
        let hit = match (&self.acceletor, &self.objects) {
            (Some(acc), _) => acc.hit(p),
            (None, Some(objs)) => objs.iter().find_map(|obj| obj.hit(p)),
            (None, None) => panic!("unexpect mode"),
        };
        hit.unwrap_or(self.background)
    }

    fn bounding_box(&self) -> Option<(Vec3f, Vec3f)> {
        match (&self.acceletor, &self.objects) {
            (Some(acc), _) => acc.bounding_box(),
            (None, Some(objs)) => match objs.len() {
                0 => None,
                _ => Some(objs.iter().fold(objs[0].bounding_box(), |(min, max), obj| {
                    let (obj_min, obj_max) = obj.bounding_box();
                    (vec::minimum(min, obj_min), vec::maximum(max, obj_max))
                })),
            },
            (None, None) => panic!("unexpect mode"),
        }
    }
}
