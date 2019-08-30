use std::fs::File;
use std::fs;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path};

use num_complex::Complex;
use structopt::*;


use failure::{Error, Fail};



#[derive(Debug, StructOpt, Clone)]
struct MandelbrotParams {
    sx: f64,
    ex: f64,
    sy: f64,
    ey: f64,
    #[structopt(long = "max-iter", default_value = "80")]
    max_iter: usize,
    width: u32,
    height: u32,

    num_subtasks: usize,
    output_dir: String,
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

struct Rect {
    startx: u32,
    starty: u32,
    endx: u32,
    endy: u32
}

struct ExecuteParams {
    start: Complex<f64>,
    pixel_step: Complex<f64>,
    max_iter: usize,
    area: Rect,
    output: String,
}

fn split(params: &MandelbrotParams) -> Vec<ExecuteParams> {
    let s = Complex::new(params.sx, params.sy);
    let e = Complex::new(params.ex, params.ey);
    let size = Complex::new(params.width as f64, params.height as f64);
    let delta = e - s;
    let scale = Complex::new(delta.re / size.re, delta.im / size.im);

    // Preapre params common for all subtasks. Create zero area to replace in future on per subtasks basis.
    let area = Rect{ startx: 0, starty: 0, endx: 0, endy: 0};
    let common_params = ExecuteParams{ start: s, pixel_step: scale, max_iter: params.max_iter, area, output: String::new() };

    let mut split_params = Vec::with_capacity(params.num_subtasks);
    for part in 0..params.num_subtasks {
        let starty = (part as u32 * params.height) / params.num_subtasks as u32;
        let endy = ((part as u32 + 1) * params.height) / params.num_subtasks as u32;

        let area = Rect{ startx: 0, starty, endx: params.width, endy};
        let output = format!("{}/out-{}-{}.png", &params.output_dir, area.starty, area.endy);

        split_params.push(ExecuteParams{area, output, ..common_params})
    }

    return split_params;
}

fn exec_to_vec(params: &ExecuteParams) -> Vec<u8> {
    let data = (params.area.starty..params.area.endy)
        .into_iter()
        .flat_map(|y| {
            let im = params.pixel_step.im * y as f64;
            (params.area.startx..params.area.endx).into_iter().map(move |x| {
                let step = Complex::new(params.pixel_step.re * x as f64, im);
                let it = mandelbrot(params.start + step, params.max_iter);
                //            println!("{}x{}: it = {}", y, x, it);
                (params.max_iter as f64 * 255f64 / it as f64) as u8
            })
        })
        .collect::<Vec<u8>>();

    return data
}

fn exec(params: &ExecuteParams) -> Vec<u8> {
    let data = exec_to_vec(params);

    let width = params.area.endx - params.area.startx;
    let height = params.area.endy - params.area.starty;

    save_file(&params.output, &data, width, height);

    return data
}

fn merge_vecs(partial_results: Vec<Vec<u8>>) -> Vec<u8> {
    partial_results.into_iter().flatten().collect::<Vec<u8>>()
}

fn merge(params: &Vec<ExecuteParams>) -> Vec<u8> {
    let partial_results = params.into_iter().map(|subtask_param|{
        load_file(&subtask_param.output)
    }).collect::<Vec<Vec<u8>>>();

    merge_vecs(partial_results)
}

#[derive(Debug, Fail)]
enum SaveFileError {
    #[fail(display = "Can't save file. Buffer size doesn't match expected width {} and height {}.", width, height)]
    NotMatchingSize {
        width: u32,
        height: u32,
    },
    #[fail(display = "Can't find parent")]
    NoParent,
}



fn save_file(output: &str, data: &Vec<u8>, width: u32, height: u32) -> Result<(), Error> {

    if data.len() != (width * height) as usize {
        return Err(SaveFileError::NotMatchingSize{ width, height })?;
    }

    let path = Path::new(output);
    let display = path.display();

    fs::create_dir_all(path.parent().ok_or(SaveFileError::NoParent)?)?;

    let file = File::create(&path)?;

    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(data)?;
    Ok(())
}

fn load_file(input: &str) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(input).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    return buf
}

fn main() {
    let opt = MandelbrotParams::from_args();

    // Split step.
    let split_params = split(&opt);

    // Execute step for all subtasks.
    for subtask_params in split_params.iter() {
        exec(&subtask_params);
    }

    // Merge step.
    let data = merge(&split_params);

    // Write result image to file.
    let output_path = Path::new(&opt.output_dir).join("out.png");

    save_file(output_path.to_str().unwrap(), &data, opt.width, opt.height);
}
