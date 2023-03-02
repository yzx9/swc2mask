use super::sdf::{Hit, SDF};
use crate::vec::{self, Vec3f};

pub struct Min {
    a: Box<dyn SDF>,
    b: Box<dyn SDF>,
    bounding_box: (Vec3f, Vec3f),
}

impl Min {
    pub fn new(a: Box<dyn SDF>, b: Box<dyn SDF>) -> Min {
        let (min_a, max_a) = a.bounding_box();
        let (min_b, max_b) = b.bounding_box();
        let bounding_box = (vec::minimum(min_a, min_b), vec::maximum(max_a, max_b));
        Self { a, b, bounding_box }
    }

    // TODO: Can a trait be extracted to allow `compose` to be reused?
    pub fn compose(a: Option<Box<dyn SDF>>, b: Option<Box<dyn SDF>>) -> Option<Box<dyn SDF>> {
        if a.is_some() && b.is_some() {
            Some(Box::new(Self::new(a.unwrap(), b.unwrap())))
        } else {
            a.or(b)
        }
    }
}

impl SDF for Min {
    fn signed_distance(&self, p: Vec3f) -> f32 {
        f32::min(self.a.signed_distance(p), self.b.signed_distance(p))
    }

    fn bounding_box(&self) -> (Vec3f, Vec3f) {
        self.bounding_box
    }

    // TODO: Can a trait be extracted to allow `isin` to be reused?
    fn isin(&self, p: Vec3f) -> bool {
        self.isin_bounding_box(p) && (self.a.isin(p) || self.b.isin(p))
    }

    fn hit(&self, p: Vec3f) -> Hit {
        let a = self.a.hit(p);
        let b = self.b.hit(p);
        if a.signed_distance <= b.signed_distance {
            a
        } else {
            b
        }
    }
}
