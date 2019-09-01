use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use num_complex::Complex;
use structopt::*;

#[derive(Debug, StructOpt, Clone)]
struct Opt {
    sx: f64,
    ex: f64,
    sy: f64,
    ey: f64,
    #[structopt(long = "max-iter", default_value = "80")]
    max_iter: usize,
    width: u32,
    height: u32,
    #[structopt(short, long, parse(from_os_str))]
    out_file: PathBuf
}

fn mandelbrot(c: Complex<f64>, max_iter: usize) -> usize {
    let mut z = Complex::from(0f64);
    let mut n = 0;
    while z.norm() <= 2f64 && n < max_iter {
        z = z * z + c;
        n += 1;
    }
    n
}

fn main() {
    let opt = Opt::from_args();

    let s = Complex::new(opt.sx, opt.sy);
    let e = Complex::new(opt.ex, opt.ey);
    let size = Complex::new(opt.width as f64, opt.height as f64);
    let delta = e - s;
    let scal = Complex::new(delta.re / size.re, delta.im / size.im);

    let data = (0..opt.height)
        .into_iter()
        .flat_map(|y| {
            let im = scal.im * y as f64;
            let opt = opt.clone();
            (0..opt.width).into_iter().map(move |x| {
                let step = Complex::new(scal.re * x as f64, im);
                let it = mandelbrot(s + step, opt.max_iter);
                //            println!("{}x{}: it = {}", y, x, it);
                (opt.max_iter as f64 * 255f64 / it as f64) as u8
            })
        })
        .collect::<Vec<u8>>();

    let display = opt.out_file.display();

    let file = match File::create(&opt.out_file) {
        Err(why) => panic!("couldn't create file {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut encoder = png::Encoder::new(BufWriter::new(file), opt.width, opt.height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
