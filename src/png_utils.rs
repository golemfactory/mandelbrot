use std::fs::File;
use std::io::{BufWriter};
use std::path::{Path};

use failure::{Error, Fail};
use png::StreamWriter;
use std::io::{Write};


#[derive(Debug, Fail)]
pub enum SaveFileError {
    #[fail(display = "Can't save file. Buffer size doesn't match expected width {} and height {}.", width, height)]
    NotMatchingSize {
        width: u32,
        height: u32,
    },
    #[fail(display = "Can't find parent")]
    NoParent,
}



pub fn save_file(output: &mut dyn Write, data: &Vec<u8>, width: u32, height: u32) -> Result<(), Error> {

    if data.len() != (width * height) as usize {
        return Err(SaveFileError::NotMatchingSize{ width, height })?;
    }

    let mut encoder = png::Encoder::new(output, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(data)?;
    Ok(())
}

pub fn load_file(input: &Path) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(input).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf).unwrap();

    return buf
}



