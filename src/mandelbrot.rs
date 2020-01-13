use std::fs;
use std::path::Path;

use num_complex::Complex;
use structopt::*;

use gwasm_dispatcher::SplitContext;
use gwasm_dispatcher::{Blob, Output, TaskResult};
use serde::{Deserialize, Serialize};

use crate::png_utils;
use crate::utils;

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
pub struct MandelbrotParams {
    #[structopt(long = "sx", default_value = "0.2")]
    sx: f64,
    #[structopt(long = "ex", default_value = "0.35")]
    ex: f64,
    #[structopt(long = "sy", default_value = "0.6")]
    sy: f64,
    #[structopt(long = "ey", default_value = "0.45")]
    ey: f64,
    #[structopt(long = "max-iter", default_value = "80")]
    max_iter: usize,
    width: u32,
    height: u32,
    num_subtasks: usize,
}

#[derive(Clone, Serialize, Deserialize)]
struct Rect {
    startx: u32,
    starty: u32,
    endx: u32,
    endy: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExecuteParams {
    #[serde(with = "ComplexDef")]
    start: Complex<f64>,
    #[serde(with = "ComplexDef")]
    pixel_step: Complex<f64>,
    max_iter: usize,
    area: Rect,
}

// This is required to run serde serialization on Complex type.
// Check https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "Complex")]
pub struct ComplexDef<T> {
    pub re: T,
    pub im: T,
}

pub struct Mandelbrot;

impl Mandelbrot {
    fn mandelbrot(c: Complex<f64>, max_iter: usize) -> usize {
        let mut z = Complex::from(0f64);
        let mut n = 0;
        while z.norm() <= 2f64 && n < max_iter {
            z = z * z + c;
            n += 1;
        }
        n
    }

    fn exec_to_vec(params: &ExecuteParams) -> Vec<u8> {
        let data = (params.area.starty..params.area.endy)
            .into_iter()
            .flat_map(|y| {
                let im = params.pixel_step.im * y as f64;
                (params.area.startx..params.area.endx)
                    .into_iter()
                    .map(move |x| {
                        let step = Complex::new(params.pixel_step.re * x as f64, im);
                        let it = Mandelbrot::mandelbrot(params.start + step, params.max_iter);
                        //            println!("{}x{}: it = {}", y, x, it);
                        (params.max_iter as f64 * 255f64 / it as f64) as u8
                    })
            })
            .collect::<Vec<u8>>();

        return data;
    }

    fn merge_vecs(partial_results: Vec<Vec<u8>>) -> Vec<u8> {
        partial_results.into_iter().flatten().collect::<Vec<u8>>()
    }

    pub fn split(context: &mut dyn SplitContext) -> Vec<(ExecuteParams, Output)> {
        let params = utils::parse_args::<MandelbrotParams>(context.args());

        let s = Complex::new(params.sx, params.sy);
        let e = Complex::new(params.ex, params.ey);
        let size = Complex::new(params.width as f64, params.height as f64);
        let delta = e - s;
        let scale = Complex::new(delta.re / size.re, delta.im / size.im);

        // Preapre params common for all subtasks. Create zero area to replace in future on per subtasks basis.
        let area = Rect {
            startx: 0,
            starty: 0,
            endx: 0,
            endy: 0,
        };
        let common_params = ExecuteParams {
            start: s,
            pixel_step: scale,
            max_iter: params.max_iter,
            area,
        };

        let mut split_params = Vec::with_capacity(params.num_subtasks);
        for part in 0..params.num_subtasks {
            let starty = (part as u32 * params.height) / params.num_subtasks as u32;
            let endy = ((part as u32 + 1) * params.height) / params.num_subtasks as u32;

            let area = Rect {
                startx: 0,
                starty,
                endx: params.width,
                endy,
            };

            split_params.push((
                ExecuteParams {
                    area,
                    ..common_params
                },
                context.new_blob(),
            ))
        }

        return split_params;
    }

    pub fn execute(params: ExecuteParams, png: Output) -> (Blob,) {
        let data = Mandelbrot::exec_to_vec(&params);

        let width = params.area.endx - params.area.startx;
        let height = params.area.endy - params.area.starty;

        png_utils::save_file(&mut png.open().unwrap(), &data, width, height).unwrap();

        return (Blob::from_output(png),);
    }

    pub fn merge(args_vec: &Vec<String>, params: TaskResult<(ExecuteParams, Output), (Blob,)>) {
        let args = utils::parse_args::<MandelbrotParams>(args_vec);

        let partial_results = params
            .into_iter()
            .map(|((_params, _), (image_blob,))| {
                png_utils::load_file(&mut image_blob.open().unwrap())
            })
            .collect::<Vec<Vec<u8>>>();

        let data = Mandelbrot::merge_vecs(partial_results);

        // Write result image to file.
        let output_path = Path::new("out.png");
        let mut out_stream = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(output_path)
            .unwrap();

        png_utils::save_file(&mut out_stream, &data, args.width, args.height).unwrap();
    }
}
