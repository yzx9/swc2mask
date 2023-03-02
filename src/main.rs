mod neuron;
mod render;
mod sdf;
mod util;
mod vec;

#[macro_use]
extern crate lazy_static;

use crate::{
    neuron::{Node, SWC},
    render::{ImageStackRenderer, Msaa, ObjectsScene, Renderer, Scene, TiffWriter},
    vec::Vec3f,
};
use clap::Parser;
use std::{cell::RefCell, rc::Rc, sync::Arc};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long, default_value_t = String::from("solid_color"))]
    mode: String,

    #[arg(long, default_value_t = 1)]
    msaa: i32,

    #[arg(long, default_value_t = false)]
    silent: bool,

    #[arg(long)]
    reset_radius: Option<f32>,

    #[arg(long)]
    node: Option<i32>,

    #[arg(long)]
    decay: Option<f32>,

    #[arg(long)]
    align: Option<String>,

    #[arg(long)]
    range: Option<String>,

    #[arg(long)]
    threads: Option<usize>,

    #[arg(long)]
    resolution: Option<String>,
}

fn main() {
    let args = Args::parse();
    let neuron = get_neuron(&args);
    let scene = get_scene(&args, neuron);
    let renderer = get_renderer(&args, scene);
    let w = get_writer(&args, renderer);
    if args.output.ends_with("/") {
        w.write_images(&args.output).expect("fails to write images")
    } else {
        w.write_image(&args.output).expect("fails to write image")
    }
}

fn get_neuron(args: &Args) -> SWC {
    if !args.silent {
        println!("read swc: {}", args.input);
    }
    let neuron = SWC::read(&args.input).expect("fails to read swc");
    if let Some(r) = args.reset_radius {
        set_radius_dfs(&neuron.root, r);
    }
    neuron
}

fn get_scene(args: &Args, neuron: SWC) -> Arc<dyn Scene> {
    let mut scene = ObjectsScene::new();
    match args.mode.as_str() {
        "solid_color" => {
            scene.add(neuron.sdf());
        }
        "path_decay" => {
            let s = Vec3f::new(0.0, 0.0, 0.0);
            let e = Vec3f::new(1.0, 1.0, 1.0);
            let decay = args.decay.expect("missing decay arg");
            let decay_fn = |a| vec::interpolate(s, e, a / decay);
            let sdfs = match args.node {
                Some(id) => {
                    let n = neuron.node(id).expect("node not found");
                    let n = n.borrow();
                    n.sdf_with_path_decay(decay_fn)
                }
                None => neuron.sdf_with_path_decay(decay_fn),
            };
            for sdf in sdfs {
                scene.add(sdf);
            }
        }
        _ => panic!("invalid mode"),
    }
    Arc::new(scene)
}

fn get_renderer(args: &Args, scene: Arc<dyn Scene>) -> Box<dyn Renderer> {
    let mut renderer = ImageStackRenderer::new(scene);
    renderer.set_msaa(Msaa::try_from(args.msaa).expect("invalid msaa"));
    if let Some(resolution) = &args.resolution {
        set_resolution(&mut renderer, resolution).unwrap();
    }

    if let Some(align) = &args.align {
        set_align(&mut renderer, align).unwrap()
    } else if let Some(range) = &args.range {
        set_range(&mut renderer, range).unwrap()
    }

    if let Some(threads) = args.threads {
        renderer.set_num_threads(threads);
    }
    Box::from(renderer)
}

fn get_writer(args: &Args, renderer: Box<dyn Renderer>) -> TiffWriter {
    let mut w = TiffWriter::new(Box::from(renderer));
    w.set_verbose(!args.silent);
    w
}

fn set_radius_dfs(n: &Rc<RefCell<Node>>, radius: f32) {
    let mut n = n.borrow_mut();
    n.radius = radius;
    for c in n.children.iter() {
        set_radius_dfs(&c, radius)
    }
}

fn set_resolution<'a>(
    renderer: &'a mut ImageStackRenderer,
    resolution: &'a str,
) -> Result<(), &'a str> {
    let res: Vec<_> = resolution
        .split(',')
        .map(|a| a.parse::<f32>().unwrap())
        .collect();
    if res.len() != 3 {
        return Err("invalid range");
    }

    renderer.set_resolution(res[0], res[1], res[2]);
    Ok(())
}

fn set_align<'a>(renderer: &'a mut ImageStackRenderer, align: &'a str) -> Result<(), &'a str> {
    if align.ends_with(".v3dpbd") {
        let mysz = util::V3DPBD::read(&align)?.mysz();
        renderer.set_range(
            Vec3f::new(0.0, 0.0, 0.0),
            Vec3f::new(mysz[0] as f32, mysz[1] as f32, mysz[2] as f32),
        );
        Ok(())
    } else {
        Err("unsupport align mode")
    }
}

fn set_range<'a>(renderer: &'a mut ImageStackRenderer, range: &'a str) -> Result<(), &'a str> {
    let ranges: Vec<_> = range
        .split(',')
        .map(|a| a.parse::<f32>().unwrap())
        .collect();
    if ranges.len() != 6 {
        return Err("invalid range");
    }

    renderer.set_range(
        Vec3f::new(ranges[0], ranges[1], ranges[2]),
        Vec3f::new(ranges[3], ranges[4], ranges[5]),
    );
    Ok(())
}
