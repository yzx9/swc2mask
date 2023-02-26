use super::{renderer::Image, Renderer};
use image::{self, ImageError};
use indicatif::ProgressIterator;
use std::{fs::File, io::BufWriter, time::Instant};
use tiff::{
    encoder::{colortype, TiffEncoder},
    TiffError,
};

pub struct TiffWriter {
    renderer: Box<dyn Renderer>,
    verbose: bool,
}

impl TiffWriter {
    pub fn new<'a>(renderer: Box<dyn Renderer>) -> TiffWriter {
        let verbose = false;
        TiffWriter { renderer, verbose }
    }

    pub fn set_verbose(&mut self, flag: bool) {
        self.verbose = flag;
    }

    pub fn write_images(self, dir: &str) -> Result<(), ImageError> {
        self.apply(|a| a.write_images_impl(dir))
    }

    fn write_images_impl(self, dir: &str) -> Result<(), ImageError> {
        for (i, img) in self.iter().enumerate() {
            img.save(format!("{dir}/{i}.tif"))?;
        }
        Ok(())
    }

    pub fn write_image(self, fname: &str) -> Result<(), TiffError> {
        self.apply(|a| a.write_image_impl(fname))
    }

    pub fn write_image_impl(self, fname: &str) -> Result<(), TiffError> {
        let file = File::create(fname)?;
        let w = &mut BufWriter::new(file); // always seekable
        let mut tiff = TiffEncoder::new(w)?;
        for img in self.iter() {
            tiff.write_image::<colortype::Gray8>(img.width(), img.height(), &img)?;
        }
        Ok(())
    }

    fn iter<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = Image> + 'a> {
        let mut iter = self.renderer.image_stack();
        if self.verbose {
            iter = Box::new(iter.progress());
        }
        iter
    }

    fn apply<R>(self, f: impl FnOnce(Self) -> R) -> R {
        if self.verbose {
            let start = Instant::now();
            let re = f(self);

            let mut ms = start.elapsed().as_millis();
            let mut sec = ms / 1000;
            ms %= 1000;
            let mut min = sec / 60;
            sec %= 60;
            let hr = min / 60;
            min %= 60;
            println!(
                "time cost: {:?} hours {:?} mintes {:?} seconds {:?} ms",
                hr, min, sec, ms
            );
            re
        } else {
            f(self)
        }
    }
}
