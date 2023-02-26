use crate::{
    render::{Object, SDFObject, VAxisLinearGradient},
    sdf::RoundCone,
    vec::Vec3f,
};
use std::{
    cell::RefCell,
    collections::HashSet,
    fmt,
    rc::{Rc, Weak},
    sync::Arc,
};

pub struct Node {
    pub id: i32,
    pub strcture: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub pid: i32,

    pub parent: Weak<RefCell<Node>>,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn xyz(&self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }

    pub fn push_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child)
    }

    pub fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.upgrade()
    }

    pub fn is_termination(&self) -> bool {
        self.children.len() == 0
    }

    pub fn is_elongation(&self) -> bool {
        self.children.len() == 1
    }

    pub fn is_bifurcation(&self) -> bool {
        self.children.len() > 1
    }

    pub fn string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}, {}, {}",
            self.id, self.strcture, self.x, self.y, self.z, self.radius, self.pid
        )
    }

    pub fn sdf_with_path_decay<F>(&self, decay_fn: F) -> Vec<Arc<dyn Object>>
    where
        F: Fn(f32) -> Vec3f,
    {
        let mut visited = HashSet::new();
        visited.insert(self.id);
        let mut out = vec![];
        Self::_sdf_with_path_decay(&self, &decay_fn, 0.0, &mut visited, &mut out);
        out
    }

    fn _sdf_with_path_decay<F>(
        &self,
        decay_fn: &F,
        cur: f32,
        visited: &mut HashSet<i32>,
        out: &mut Vec<Arc<dyn Object>>,
    ) where
        F: Fn(f32) -> Vec3f,
    {
        let mut ns = vec![];
        if let Some(p) = self.parent() {
            if visited.insert(p.borrow().id) {
                ns.push(p);
            }
        }
        for c in self.children.iter() {
            if visited.insert(c.borrow().id) {
                ns.push(c.clone());
            }
        }

        let a = decay_fn(cur);
        let a_norm = a.norm();
        for n in ns {
            let n = n.borrow();
            let acc = cur + (self.xyz() - n.xyz()).norm();
            let b = decay_fn(acc);

            const EPS: f32 = 1e-6;
            if a_norm > EPS && b.norm() > EPS {
                let m = VAxisLinearGradient::new(a, b);
                let link = Box::new(RoundCone::new(self.xyz(), self.radius, n.xyz(), n.radius)); // TODO: perf
                let obj = SDFObject::new(link, m);
                out.push(obj);
            }

            Self::_sdf_with_path_decay(&n, decay_fn, acc, visited, out);
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.string())
    }
}
