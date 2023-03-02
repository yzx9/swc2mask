use super::{error_kind::RootNotFoundError, node::Node};
use crate::{
    render::{Material, Object, SDFObject, SolidColor},
    sdf::{Min, RoundCone, Sphere, SDF},
    vec::Vec3f,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fmt, fs,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    rc::{Rc, Weak},
    sync::Arc,
};

pub struct SWC {
    pub root: Rc<RefCell<Node>>,
    pub count: usize,
}

const ROOT_ID: i32 = 1;

impl SWC {
    pub fn read(fname: &str) -> Result<Self, Box<dyn Error>> {
        let fname = fs::canonicalize(&PathBuf::from(fname))?;
        let file = File::open(fname)?;
        let mut nodes = HashMap::<i32, Rc<RefCell<Node>>>::new();
        for line in BufReader::new(file).lines() {
            let line = line?;
            if line.is_empty() || line.starts_with("#") {
                continue; // skip comment line
            }

            let cells: Vec<&str> = line.split(" ").collect();
            if let [id, structure, x, y, z, radius, pid, ..] = cells[..] {
                let id: i32 = id.parse()?;
                let pid: i32 = pid.parse()?;
                let parent = match pid {
                    -1 => None,
                    _ => nodes.get_mut(&pid).or_else(|| {
                        println!("parent not found, id: {id}, pid: {pid}");
                        None
                    }),
                };

                let node = Rc::new(RefCell::new(Node {
                    id,
                    strcture: structure.parse()?,
                    x: x.parse()?,
                    y: y.parse()?,
                    z: z.parse()?,
                    radius: radius.parse()?,
                    pid,

                    parent: match &parent {
                        Some(p) => Rc::downgrade(&p),
                        None => Weak::new(),
                    },
                    children: Vec::new(),
                }));

                if let Some(p) = parent {
                    (*p.borrow_mut()).push_child(Rc::clone(&node));
                }
                nodes.insert(id, node);
            };
        }

        Ok(SWC {
            root: nodes.remove(&ROOT_ID).ok_or(Box::new(RootNotFoundError))?,
            count: nodes.len(),
        })
    }

    pub fn node(&self, id: i32) -> Option<Rc<RefCell<Node>>> {
        Self::_node(self.root.clone(), id)
    }

    pub fn sdf(&self) -> Arc<dyn Object> {
        self.sdf_with_material(SolidColor::new(Vec3f::new(1.0, 1.0, 1.0)))
    }

    pub fn sdf_with_material(&self, material: Arc<dyn Material>) -> Arc<dyn Object> {
        let n = &self.root.borrow();
        let sdf = Self::_sdf_with_material(n).unwrap_or(Box::new(Sphere::new(n.xyz(), n.radius)));
        SDFObject::new(Box::from(sdf), material)
    }

    pub fn sdf_with_path_decay<F>(&self, decay_fn: F) -> Vec<Arc<dyn Object>>
    where
        F: Fn(f32) -> Vec3f,
    {
        self.root.borrow().sdf_with_path_decay(decay_fn)
    }

    fn _node(n: Rc<RefCell<Node>>, id: i32) -> Option<Rc<RefCell<Node>>> {
        let nn = n.borrow();
        if nn.id == id {
            return Some(n.clone());
        }

        for c in &nn.children {
            if let Some(a) = Self::_node(c.clone(), id) {
                return Some(a);
            }
        }
        None
    }

    fn _sdf_with_material(n: &Node) -> Option<Box<dyn SDF>> {
        n.children.iter().fold(None, |sdf, c| {
            let c = c.borrow();
            let link = Box::new(RoundCone::new(n.xyz(), n.radius, c.xyz(), c.radius));
            let child = Min::compose(Some(link), Self::_sdf_with_material(&c));
            Min::compose(sdf, child)
        })
    }
}

impl fmt::Display for SWC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neuron with {} nodes", self.count)
    }
}
