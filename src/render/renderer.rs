use super::{anti_aliasing::MSAA_OPTIONS, Msaa, Scene};
use crate::vec::Vec3f;
use image::{ImageBuffer, Luma};
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;

pub type Image = image::ImageBuffer<Luma<u8>, Vec<u8>>;
pub type Images<'a> = Box<dyn ExactSizeIterator<Item = Image> + 'a>;

pub trait Renderer {
    fn scene(&self) -> Arc<dyn Scene>;
    fn image_stack(&self) -> Images;
}

pub struct ImageStackRenderer {
    scene: Arc<dyn Scene>,
    resolution: Vec3f, // voxel per um
    range: Option<(Vec3f, Vec3f)>,
    msaa: Msaa,
    num_threads: Option<usize>,
}

impl ImageStackRenderer {
    pub fn new(scene: Arc<dyn Scene>) -> ImageStackRenderer {
        ImageStackRenderer {
            scene,
            resolution: Vec3f::new(1.0, 1.0, 1.0),
            range: None,
            msaa: Msaa::Disable,
            num_threads: None,
        }
    }

    pub fn set_resolution(&mut self, x: f32, y: f32, z: f32) {
        self.resolution = Vec3f::new(x, y, z);
    }

    pub fn set_range(&mut self, min: Vec3f, max: Vec3f) {
        self.range = Some((min, max));
    }

    pub fn set_msaa(&mut self, msaa: Msaa) {
        self.msaa = msaa;
    }

    pub fn set_num_threads(&mut self, num_threads: usize) {
        self.num_threads = Some(num_threads);
    }

    fn range(&self) -> (Vec3f, Vec3f) {
        self.range.or_else(|| self.scene.bounding_box()).unwrap_or((
            Vec3f::new(-100.0, -100.0, -100.0),
            Vec3f::new(100.0, 100.0, 100.0),
        )) // or panic?
    }

    fn num_threads(&self) -> usize {
        self.num_threads.unwrap_or(num_cpus::get())
    }
}

impl Renderer for ImageStackRenderer {
    fn scene(&self) -> Arc<dyn Scene> {
        Arc::clone(&self.scene)
    }

    fn image_stack<'a>(&'a self) -> Images<'a> {
        let (min, max) = self.range.unwrap_or(self.range());
        Box::new(ImageStackRendererIterator {
            renderer: self,
            min,
            max,
            width: f32::ceil((max.x - min.x) / self.resolution.x) as u32,
            height: f32::ceil((max.y - min.y) / self.resolution.y) as u32,
            frames: f32::ceil((max.z - min.z) / self.resolution.z) as u32,
            msaa: MSAA_OPTIONS.get(&self.msaa).unwrap(),
            pool: ThreadPool::new(self.num_threads()),
            i: 0,
        })
    }
}

pub struct ImageStackRendererIterator<'a> {
    renderer: &'a ImageStackRenderer,
    min: Vec3f,
    max: Vec3f,
    width: u32,
    height: u32,
    frames: u32,
    msaa: &'static [Vec3f],
    pool: ThreadPool,
    i: u32,
}

impl Iterator for ImageStackRendererIterator<'_> {
    type Item = image::ImageBuffer<Luma<u8>, Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.frames {
            return None;
        }

        let msaa = self.msaa;
        let (w, h) = (self.width, self.height);
        let mx = self.min.x;
        let r = self.renderer.resolution;

        const K_TASK: usize = 4;
        let h_per_task = f32::ceil(h as f32 / (K_TASK * self.pool.max_count()) as f32) as u32;
        let num_tasks = f32::ceil(h as f32 / h_per_task as f32) as u32;
        let z = self.min.z + r.z * self.i as f32;
        let (tx, rx) = mpsc::channel();
        for i in 0..num_tasks {
            let prev = h - h_per_task * i as u32;
            let h = u32::min(h_per_task, prev);
            let my = self.min.y + (prev - h) as f32;

            let tx = tx.clone();
            let scene = Arc::clone(&self.renderer.scene);
            self.pool.execute(move || {
                let img = ImageBuffer::from_fn(w, h, |x, y| {
                    let p = Vec3f::new(mx + r.x * x as f32, my + r.y * (h - y) as f32, z);
                    let luma = msaa
                        .as_ref()
                        .into_iter()
                        .fold(0.0, |acc, v| acc + to_luma(scene.hit(p + r * v)));
                    Luma([f32::round(255.0 * luma / msaa.len() as f32) as u8])
                });
                tx.send((i, img)).unwrap();
            });
        }
        self.i += 1;
        drop(tx);

        let mut parts: Vec<_> = rx.into_iter().collect();
        parts.sort_by(|a, b| a.0.cmp(&b.0));
        let img = parts
            .into_iter()
            .map(|a| a.1.into_raw())
            .collect::<Vec<_>>()
            .concat();
        ImageBuffer::<Luma<u8>, Vec<u8>>::from_raw(self.width, self.height, img)
    }
}

impl ExactSizeIterator for ImageStackRendererIterator<'_> {
    fn len(&self) -> usize {
        f32::ceil((self.max.z - self.min.z) / self.renderer.resolution.y) as usize
    }
}

fn to_luma(c: Vec3f) -> f32 {
    0.2126 * c.x + 0.7152 * c.y + 0.0722 * c.z
}
